package samples.concurrency.threads;

import java.util.concurrent.atomic.AtomicInteger;

/**
 * Exercises tricky edge cases of the Thread implementation. Every scenario is deterministic and
 * bounded: on a regression it throws (fails fast with a diagnostic) rather than hanging.
 */
public class ThreadEdgeCasesDemo {

    public static void main(String[] args) throws Exception {
        reentrantWaitReleasesAllLevels();
        interruptSemantics();
        timedWaitTimesOut();
        blockedStateWhileContending();
        doubleStartThrows();
        atomicCasUnderContention();
        System.out.println("all edge cases passed");
    }

    // ---- helpers ---------------------------------------------------------------------------------

    static long deadline(long ms) {
        return System.nanoTime() + ms * 1_000_000L;
    }

    static boolean beforeDeadline(long dl) {
        return System.nanoTime() < dl;
    }

    /** Bounded wait; caller must hold `o`. Returns false once the deadline has passed. */
    static boolean waitUntil(Object o, long dl) throws InterruptedException {
        long remMs = (dl - System.nanoTime()) / 1_000_000L;
        if (remMs <= 0) return false;
        o.wait(remMs);
        return true;
    }

    static void joinOrFail(Thread t, long ms, String what) throws InterruptedException {
        t.join(ms);
        if (t.isAlive()) throw new IllegalStateException(what + ": thread did not finish in " + ms + "ms");
    }

    // ---- 1: reentrant wait must release ALL recursion levels and restore the depth ---------------

    static final Object r = new Object();
    static boolean rReady = false;
    static boolean rGo = false;

    static void reentrantWaitReleasesAllLevels() throws Exception {
        Thread w = new Thread(() -> {
            synchronized (r) {
                synchronized (r) {                       // reentrant, depth 2
                    rReady = true;
                    r.notifyAll();
                    while (!rGo) {
                        try {
                            r.wait();                    // must fully release BOTH levels here
                        } catch (InterruptedException e) {
                            throw new RuntimeException(e);
                        }
                    }
                    if (!Thread.holdsLock(r)) throw new IllegalStateException("lost lock (depth 2) after wait");
                }
                if (!Thread.holdsLock(r)) throw new IllegalStateException("lost lock (depth 1) after wait");
            }
            if (Thread.holdsLock(r)) throw new IllegalStateException("still holds lock after synchronized");
        }, "reentrant-worker");

        w.start();
        // Acquirable only once the worker's wait() released *every* reentrant level it held.
        synchronized (r) {
            long dl = deadline(3000);
            while (!rReady) {
                if (!waitUntil(r, dl)) throw new IllegalStateException("worker never became ready");
            }
            rGo = true;
            r.notifyAll();
        }
        joinOrFail(w, 3000, "reentrant-wait");
        System.out.println("1 reentrant-wait ok");
    }

    // ---- 2: interrupt status + interruptible wait ------------------------------------------------

    static void interruptSemantics() {
        Thread me = Thread.currentThread();

        me.interrupt();
        if (!me.isInterrupted()) throw new IllegalStateException("isInterrupted should be true");
        if (!me.isInterrupted()) throw new IllegalStateException("isInterrupted must not clear");
        if (!Thread.interrupted()) throw new IllegalStateException("interrupted() should observe the flag");
        if (Thread.interrupted()) throw new IllegalStateException("interrupted() must clear the flag");

        // Interrupt set before waiting: wait() throws immediately and clears the flag.
        me.interrupt();
        boolean threw = false;
        Object o = new Object();
        synchronized (o) {
            try {
                o.wait();
                throw new IllegalStateException("wait did not throw on a preset interrupt");
            } catch (InterruptedException e) {
                threw = true;
            }
        }
        if (!threw) throw new IllegalStateException("expected InterruptedException");
        if (me.isInterrupted()) throw new IllegalStateException("InterruptedException must clear the flag");
        System.out.println("2 interrupt-semantics ok");
    }

    // ---- 3: timed wait returns on timeout when nobody notifies -----------------------------------

    static void timedWaitTimesOut() throws Exception {
        Object o = new Object();
        long start = System.nanoTime();
        synchronized (o) {
            o.wait(80);
        }
        long elapsedMs = (System.nanoTime() - start) / 1_000_000L;
        if (elapsedMs < 40) throw new IllegalStateException("timed wait returned too early: " + elapsedMs + "ms");
        if (elapsedMs > 5000) throw new IllegalStateException("timed wait far too slow: " + elapsedMs + "ms");
        System.out.println("3 timed-wait ok");
    }

    // ---- 4: a thread contending for a held monitor reports BLOCKED -------------------------------

    static final Object b = new Object();
    static volatile boolean holderAcquired = false;
    static volatile boolean release = false;

    static void blockedStateWhileContending() throws Exception {
        holderAcquired = false;
        release = false;
        Thread holder = new Thread(() -> {
            synchronized (b) {
                holderAcquired = true;
                while (!release) {
                    try {
                        Thread.sleep(5);                 // sleep does NOT release the monitor
                    } catch (InterruptedException e) {
                        throw new RuntimeException(e);
                    }
                }
            }
        }, "holder");
        holder.start();

        long dl0 = deadline(3000);
        while (!holderAcquired) {
            if (!beforeDeadline(dl0)) throw new IllegalStateException("holder never acquired the monitor");
            Thread.sleep(1);
        }

        Thread blocker = new Thread(() -> {
            synchronized (b) {                           // blocks: holder owns b
            }
        }, "blocker");
        blocker.start();

        long dl = deadline(3000);
        while (blocker.getState() != Thread.State.BLOCKED) {
            if (!beforeDeadline(dl)) {
                throw new IllegalStateException("blocker never reached BLOCKED; state=" + blocker.getState());
            }
            Thread.sleep(1);
        }

        release = true;
        joinOrFail(holder, 3000, "holder");
        joinOrFail(blocker, 3000, "blocker");
        System.out.println("4 blocked-state ok");
    }

    // ---- 5: starting a thread twice is rejected --------------------------------------------------

    static void doubleStartThrows() throws Exception {
        Thread t = new Thread(() -> {}, "once");
        t.start();
        joinOrFail(t, 3000, "double-start");
        boolean threw = false;
        try {
            t.start();
        } catch (IllegalThreadStateException e) {
            threw = true;
        }
        if (!threw) throw new IllegalStateException("second start() must throw IllegalThreadStateException");
        System.out.println("5 double-start ok");
    }

    // ---- 6: AtomicInteger under contention loses no updates (Unsafe CAS) --------------------------

    static void atomicCasUnderContention() throws Exception {
        AtomicInteger counter = new AtomicInteger(0);
        // Kept modest: each increment is an interpreted getAndAddInt CAS-retry loop that serializes
        // on the object's lock, so a large count is slow (esp. on CI) without adding detection power
        // — even this many contended increments exposes a non-atomic CAS as lost updates.
        int nThreads = 2;
        int perThread = 5_000;
        Thread[] ts = new Thread[nThreads];
        for (int i = 0; i < nThreads; i++) {
            ts[i] = new Thread(() -> {
                for (int j = 0; j < perThread; j++) {
                    counter.incrementAndGet();
                }
            }, "cas-" + i);
            ts[i].start();
        }
        for (Thread t : ts) {
            joinOrFail(t, 180_000, "atomic-cas");
        }
        int expected = nThreads * perThread;
        if (counter.get() != expected) {
            throw new IllegalStateException("atomic lost updates: " + counter.get() + " != " + expected);
        }
        System.out.println("6 atomic-cas ok");
    }
}
