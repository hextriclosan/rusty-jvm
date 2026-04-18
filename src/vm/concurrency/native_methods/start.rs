//! Purpose: Implements the native backend for `java.lang.Thread.start0()`.
//!
//! Implementation Details:
//! Maps the Java thread start instruction directly to `tokio::spawn`.
//! It extracts the `run()` method from the `java.lang.Thread` object, creates a
//! new stack frame, and hands ownership of that frame to a new Tokio asynchronous task.

use crate::vm::error::Result;
use crate::vm::stack::stack_frame::StackFrames;
use crate::vm::execution_engine::engine::Engine;
use crate::vm::heap::heap::HEAP;
use crate::vm::concurrency::task_local::CURRENT_JAVA_THREAD_ID;
use crate::vm::concurrency::thread_manager::register_thread;

/// Spawns a new virtual thread (Tokio task) for the given Java Thread.
///
/// Java signature: `private native void start0()`
pub async fn start0(args: &[i32], _stack_frames: &mut StackFrames) -> Result<Vec<i32>> {
    let thread_ref = args[0];

    // 1. Locate the `run()` method on this Thread object.
    let class_name = HEAP.get_instance_name(thread_ref)?;
    
    // Use `lookup_method` to properly find `run:()V` even if it is inherited from a superclass.
    let run_method = crate::vm::method_area::lookup::lookup_method(&class_name, "run:()V")?
        .ok_or_else(|| crate::vm::error::Error::new_execution(&format!("Method run:()V not found in {}", class_name)))?;
    
    let mut new_stack_frame = run_method.new_stack_frame()?;
    
    // The `this` reference for `run()` is the thread object itself.
    new_stack_frame.set_local(0, thread_ref);

    // 2. Spawn the Tokio task.
    let handle = tokio::spawn(async move {
        // Bind the Java Thread ID to this Tokio task.
        let result = CURRENT_JAVA_THREAD_ID.scope(thread_ref, async move {
            // Execute the JVM interpreter loop for this thread!
            Engine::execute(new_stack_frame, "Thread.run()").await
        }).await;

        if let Err(e) = result {
            tracing::error!("Virtual Thread {} crashed: {}", thread_ref, e);
        }
        
        // Clean up registry when done
        crate::vm::concurrency::thread_manager::unregister_thread(thread_ref);
    });

    // Register the thread handle
    register_thread(thread_ref, handle);

    Ok(vec![])
}