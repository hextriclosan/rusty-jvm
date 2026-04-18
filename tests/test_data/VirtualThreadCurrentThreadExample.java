package samples.concurrency.virtualthreads;

public class VirtualThreadCurrentThreadExample {
    static volatile Thread t1Ref;
    static volatile Thread t2Ref;

    public static void main(String[] args) {
        Thread mainThread = Thread.currentThread();

        Thread t1 = new Thread(() -> {
            t1Ref = Thread.currentThread();
        });
        Thread t2 = new Thread(() -> {
            t2Ref = Thread.currentThread();
        });

        t1.start();
        t2.start();

        // Give virtual threads time to execute
        try { Thread.sleep(500); } catch (InterruptedException e) {}

        System.out.println("t1 is not main: " + (t1Ref != mainThread));
        System.out.println("t2 is not main: " + (t2Ref != mainThread));
        System.out.println("t1 is not t2: " + (t1Ref != t2Ref));
        
        // Ensure that the task-local reference perfectly matches the object we spawned
        System.out.println("t1Ref == t1: " + (t1Ref == t1));
        System.out.println("t2Ref == t2: " + (t2Ref == t2));
    }
}
