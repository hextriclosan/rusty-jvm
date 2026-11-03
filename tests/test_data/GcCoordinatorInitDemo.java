package samples.gc;

import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;

public class GcCoordinatorInitDemo {
    static final CountDownLatch entered = new CountDownLatch(1);
    static final CountDownLatch release = new CountDownLatch(1);
    static final CountDownLatch contenderReady = new CountDownLatch(1);
    static int initializations;
    static volatile Throwable workerFailure;

    static class Recursive {
        static int value = initialize();
        static int initialize() {
            return read() + 41;
        }
        static int read() {
            return value;
        }
    }

    static class Contended {
        static int value = initialize();
        static int initialize() {
            initializations++;
            entered.countDown();
            await(release);
            return 42;
        }
    }

    static class Failed {
        static int value = initialize();
        static int initialize() {
            throw new IllegalStateException("initializer failed");
        }
    }

    static void await(CountDownLatch latch) {
        try {
            if (!latch.await(10, TimeUnit.SECONDS)) {
                throw new AssertionError("initialization event timed out");
            }
        } catch (InterruptedException e) {
            throw new AssertionError(e);
        }
    }

    public static void main(String[] args) throws InterruptedException {
        if (Recursive.value != 41) throw new AssertionError("recursive init");
        System.out.println("recursive-ok");
        Thread owner = new Thread(() -> {
            try {
                if (Contended.value != 42) throw new AssertionError("owner value");
            } catch (Throwable e) {
                workerFailure = e;
            }
        });
        owner.start();
        await(entered);
        Thread contender = new Thread(() -> {
            contenderReady.countDown();
            try {
                if (Contended.value != 42) throw new AssertionError("contender value");
            } catch (Throwable e) {
                workerFailure = e;
            }
        });
        contender.start();
        await(contenderReady);
        release.countDown();
        owner.join(10000);
        contender.join(10000);
        if (owner.isAlive() || contender.isAlive()) throw new AssertionError("init deadlock");
        if (workerFailure != null) throw new AssertionError(workerFailure);
        if (initializations != 1) throw new AssertionError("duplicate initializer");
        System.out.println("contended-ok");
        try {
            int ignored = Failed.value;
            throw new AssertionError("first failed initialization succeeded");
        } catch (ExceptionInInitializerError error) {
            if (!(error.getCause() instanceof IllegalStateException)) throw error;
        }
        for (int i = 0; i < 2; i++) {
            try {
                int ignored = Failed.value;
                throw new AssertionError("failed class became initialized");
            } catch (NoClassDefFoundError expected) {
                if (expected.getCause() == null) throw new AssertionError("lost cause");
            }
        }
        System.out.println("failure-ok");
    }
}
