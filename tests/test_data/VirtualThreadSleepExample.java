package samples.concurrency.virtualthreads;

public class VirtualThreadSleepExample {
    public static void main(String[] args) {
        Thread t1 = new Thread(() -> {
            try {
                Thread.sleep(500);
                System.out.println("Thread 1 woke up");
            } catch (InterruptedException e) { }
        });

        Thread t2 = new Thread(() -> {
            try {
                Thread.sleep(100);
                System.out.println("Thread 2 woke up");
            } catch (InterruptedException e) { }
        });

        t1.start();
        t2.start();

        try {
            // Keep the main thread alive long enough for virtual threads to finish.
            // Because rusty-jvm uses Tokio, the sleep calls will execute asynchronously, 
            // causing t2 to wake up and print BEFORE t1, proving concurrency!
            Thread.sleep(1000);
        } catch (InterruptedException e) { }
        
        System.out.println("Main thread finished");
    }
}