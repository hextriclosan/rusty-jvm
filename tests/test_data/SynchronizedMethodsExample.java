package samples.concurrency.threads;

/**
 * Methods declared {@code synchronized} lock their receiver, or their class for a static method, for
 * the duration of the call, exactly like a {@code synchronized} block does: the monitor is held
 * inside the method and is reentrant, it is released on return and when an exception leaves the
 * method, it excludes other threads, and it is the one {@code wait} and {@code notify} need.
 */
public class SynchronizedMethodsExample {
    private int counter;
    private static int staticCounter;
    private Integer handoff;

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
        example.put(42);
        taker.join();
    }
}
