//! Purpose: Provides task-local storage to associate Tokio tasks with JVM thread identifiers.
//!
//! Implementation Details:
//! Uses `tokio::task_local!` to securely bind a Java thread ID (`i32` reference to the heap)
//! to the currently executing asynchronous task. This replaces standard OS-level thread-local 
//! storage (TLS) since Tokio multiplexes many tasks onto a single OS thread.

tokio::task_local! {
    /// The JVM heap reference of the `java.lang.Thread` object currently executing.
    pub static CURRENT_JAVA_THREAD_ID: i32;
}