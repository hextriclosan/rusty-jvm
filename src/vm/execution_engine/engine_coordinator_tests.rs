use super::*;
use crate::vm::execution_engine::opcode::RETURN;
use crate::vm::safepoint::{self, Event, COORDINATOR};
use crate::vm::stack::stack_frame::ExceptionTable;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::time::Duration;

fn single_return_frame() -> StackFrame {
    StackFrame::new(
        Arc::new("poll:()V".to_string()),
        0,
        0,
        Arc::new(vec![RETURN]),
        Arc::new("PollBudget".to_string()),
        Arc::default(),
        Arc::new(ExceptionTable::new(Vec::new())),
    )
}

/// The poll budget must span invocations: it used to be a local of each `Engine::execute`,
/// restarting at zero on every call, so a chain of short invocations could run unbounded
/// bytecodes without ever polling - and a thread that never polls hangs a stop-the-world.
#[test]
fn gc_coordinator_nested_engine_invocations_share_one_poll_budget() {
    // Given this thread is a registered participant, targetable by reference.
    safepoint::attach();
    let target = safepoint::current().expect("attached participant");
    COORDINATOR.bind(target, 77);
    let (published, publication) = channel();
    // Subscribed before the request is released, so the trigger cannot outrun the observer.
    safepoint::observe(move |event| {
        if event == Event::SnapshotPublished {
            let _ = published.send(());
        }
    });

    // When an operation requests this thread while it runs only single-bytecode invocations.
    // The request is the real dump election, whose preserved two-second timeout bounds this
    // test: a target that never polls makes the assertion below fail instead of hanging.
    let dumper = std::thread::spawn(|| {
        safepoint::attach();
        let owner = safepoint::current().expect("attached participant");
        let _timed_out = COORDINATOR.dump(owner, 77, JavaThread::snapshot_roots(), |_| {});
        safepoint::retire();
    });

    for _ in 0..(2 * 4096) {
        Engine::execute(single_return_frame(), "poll budget").expect("single return");
    }

    // Then the interpreter published a snapshot without any one invocation being long.
    assert!(
        publication.recv_timeout(Duration::from_secs(5)).is_ok(),
        "a chain of single-bytecode invocations never reached a safepoint"
    );
    dumper.join().expect("dumper finished");
    safepoint::retire();
}
