package samples.concurrency.threads;

/**
 * Methods declared {@code synchronized} lock their receiver, or their class for a static method, for
 * the duration of the call, exactly like a {@code synchronized} block does: the monitor is held
 * inside the method and is reentrant, it is released on return and when an exception leaves the
 * method, even one the thread dies of, it excludes other threads, and it is the one {@code wait}
 * and {@code notify} need.
 */
public class SynchronizedMethodsExample {
    private int counter;
    private static int staticCounter;
    private Integer handoff;
    private static volatile boolean probed;

    /** Fails to initialize; its first use, inside a synchronized method, kills that thread. */
    static class Broken {
        static final int VALUE = fail();

        static int fail() {
            throw new IllegalStateException("broken <clinit>");
        }

        static void touch() {
        }
    }

    synchronized boolean holdsOwnLock() {
        return Thread.holdsLock(this);
    }

    static synchronized boolean holdsClassLock() {
        return Thread.holdsLock(SynchronizedMethodsExample.class);
    }

    synchronized boolean[] reenter() {
        boolean[] held = new boolean[3];
        held[0] = Thread.holdsLock(this);
        held[1] = holdsOwnLock(); // a nested synchronized call on the same receiver
        held[2] = Thread.holdsLock(this); // still held once the nested call has returned
        return held;
    }

    synchronized void fail() {
        throw new IllegalStateException("leaving through an exception");
    }

    synchronized void triggerBrokenInit() {
        Broken.touch();
    }

    synchronized void increment() {
        counter++;
    }

    static synchronized void incrementStatic() {
        staticCounter++;
    }

    synchronized void put(int value) {
        handoff = value;
        notifyAll();
    }

    synchronized int take() throws InterruptedException {
        while (handoff == null) {
            wait(); // only legal with the monitor this method holds
        }
        return handoff;
    }

    public static void main(String[] args) throws InterruptedException {
        SynchronizedMethodsExample example = new SynchronizedMethodsExample();
        System.out.println("held inside instance method: " + example.holdsOwnLock());
        System.out.println("held inside static method: " + holdsClassLock());
        System.out.println("held after return: " + Thread.holdsLock(example) + " "
                + Thread.holdsLock(SynchronizedMethodsExample.class));
        boolean[] reentered = example.reenter();
        System.out.println("reentrant: " + reentered[0] + " " + reentered[1] + " " + reentered[2]);
        try {
            example.fail();
        } catch (IllegalStateException e) {
            System.out.println("released after exception: " + !Thread.holdsLock(example));
        }

        // The worker dies of an error raised by the VM itself, a failed class initialization, while
        // inside a synchronized method: the monitor must still be given up with its frame.
        Thread victim = new Thread(example::triggerBrokenInit);
        victim.setUncaughtExceptionHandler((thread, error) -> System.out.println("worker died: true"));
        victim.start();
        victim.join();
        Thread prober = new Thread(() -> probed = example.holdsOwnLock());
        prober.start();
        prober.join(5_000);
        boolean entered = prober.getState() == Thread.State.TERMINATED && probed;
        System.out.println("entered after worker died: " + entered);
        if (!entered) {
            System.exit(1); // the prober is blocked for good; don't wait for it at exit
        }

        Thread[] workers = new Thread[4];
        for (int i = 0; i < workers.length; i++) {
            workers[i] = new Thread(() -> {
                for (int j = 0; j < 25_000; j++) {
                    example.increment();
                    incrementStatic();
                }
            });
            workers[i].start();
        }
        for (Thread worker : workers) {
            worker.join();
        }
        System.out.println("counters: " + example.counter + " " + staticCounter);

        Thread taker = new Thread(() -> {
            try {
                System.out.println("taken: " + example.take());
            } catch (InterruptedException e) {
                throw new IllegalStateException(e);
            }
        });
        taker.start();
        awaitState(taker, Thread.State.WAITING); // so that take() really waits, and is woken up
        example.put(42);
        taker.join();
    }

    private static void awaitState(Thread thread, Thread.State state) {
        long deadline = System.nanoTime() + 5_000_000_000L;
        while (thread.getState() != state) {
            if (System.nanoTime() >= deadline) {
                throw new IllegalStateException(
                        "thread never reached " + state + "; last state=" + thread.getState());
            }
            Thread.onSpinWait();
        }
    }
}
