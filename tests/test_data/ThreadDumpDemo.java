package samples.concurrency.threads;

public class ThreadDumpDemo {
    static volatile boolean stop = false;
    static volatile boolean running = false;

    public static void main(String[] args) throws InterruptedException {
        Thread worker = new Spinner();
        worker.start();
        while (!running) {
            Thread.sleep(1);
        }

        // Dump another thread's stack via a safepoint. Retry (bounded) until it catches the worker
        // inside spin(), the method it loops in — a safepoint could momentarily land elsewhere.
        String top = "none";
        long deadline = System.nanoTime() + 5_000_000_000L;
        while (System.nanoTime() < deadline) {
            StackTraceElement[] st = worker.getStackTrace();
            if (st.length > 0 && st[0].getMethodName().equals("spin")) {
                top = st[0].getClassName() + "." + st[0].getMethodName();
                break;
            }
            Thread.sleep(1);
        }

        stop = true;
        worker.join();
        System.out.println("top=" + top);
    }

    static class Spinner extends Thread {
        @Override
        public void run() {
            spin();
        }

        void spin() {
            running = true;
            long x = 0;
            while (!stop) {
                x++;
                if (x == Long.MAX_VALUE) x = 0;
            }
        }
    }
}
