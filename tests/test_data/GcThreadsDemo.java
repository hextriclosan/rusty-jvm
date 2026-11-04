package samples.gc;

import java.lang.ref.Cleaner;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicInteger;

public final class GcThreadsDemo {
    private static final int WORKERS = 12;
    private static final int COLD_CLASSES = WORKERS;
    private static final CountDownLatch READY = new CountDownLatch(WORKERS);
    private static final CountDownLatch RELEASE = new CountDownLatch(1);
    private static final CountDownLatch CLASS_STARTED = new CountDownLatch(COLD_CLASSES);
    private static final CountDownLatch RELEASE_CLASS = new CountDownLatch(1);
    private static final AtomicInteger SURVIVED = new AtomicInteger();
    private static final AtomicInteger LOADED = new AtomicInteger();
    private static final int CLEANABLES_PER_WORKER = 32;
    private static final Cleaner CLEANER = Cleaner.create();
    private static final CountDownLatch CLEANED =
            new CountDownLatch(WORKERS * CLEANABLES_PER_WORKER);

    private static final class Marker {
        private final int value;

        Marker(int value) {
            this.value = value;
        }
    }

    private static final class Cold0 { static { coldClassStarted(); } static void touch() {} }
    private static final class Cold1 { static { coldClassStarted(); } static void touch() {} }
    private static final class Cold2 { static { coldClassStarted(); } static void touch() {} }
    private static final class Cold3 { static { coldClassStarted(); } static void touch() {} }
    private static final class Cold4 { static { coldClassStarted(); } static void touch() {} }
    private static final class Cold5 { static { coldClassStarted(); } static void touch() {} }
    private static final class Cold6 { static { coldClassStarted(); } static void touch() {} }
    private static final class Cold7 { static { coldClassStarted(); } static void touch() {} }
    private static final class Cold8 { static { coldClassStarted(); } static void touch() {} }
    private static final class Cold9 { static { coldClassStarted(); } static void touch() {} }
    private static final class Cold10 { static { coldClassStarted(); } static void touch() {} }
    private static final class Cold11 { static { coldClassStarted(); } static void touch() {} }

    private static void collectWithGarbage() {
        byte[] unreachable = new byte[4096];
        unreachable[0] = 1;
        unreachable = null;
        System.gc();
    }

    private static final class LocalOnlyWorker extends Thread {
        private final int id;

        LocalOnlyWorker(int id) {
            super("gc-local-only-" + id);
            this.id = id;
        }

        @Override
        public void run() {
            Marker onlyLocal = new Marker(1000 + id);
            byte[] unreachable = new byte[8192];
            unreachable[0] = (byte) id;
            unreachable = null;
            READY.countDown();
            try {
                if (!RELEASE.await(30, TimeUnit.SECONDS)) {
                    throw new AssertionError("local-only worker was not released");
                }
            } catch (InterruptedException error) {
                throw new AssertionError(error);
            }
            if (onlyLocal.value == 1000 + id) {
                SURVIVED.incrementAndGet();
            }
        }
    }

    private static void coldClassStarted() {
        CLASS_STARTED.countDown();
        try {
            if (!RELEASE_CLASS.await(30, TimeUnit.SECONDS)) {
                throw new ExceptionInInitializerError("class-loading rendezvous was not released");
            }
        } catch (InterruptedException error) {
            throw new ExceptionInInitializerError(error);
        }
    }

    private static void loadCold(int id) {
        switch (id % COLD_CLASSES) {
            case 0: Cold0.touch(); break;
            case 1: Cold1.touch(); break;
            case 2: Cold2.touch(); break;
            case 3: Cold3.touch(); break;
            case 4: Cold4.touch(); break;
            case 5: Cold5.touch(); break;
            case 6: Cold6.touch(); break;
            case 7: Cold7.touch(); break;
            case 8: Cold8.touch(); break;
            case 9: Cold9.touch(); break;
            case 10: Cold10.touch(); break;
            case 11: Cold11.touch(); break;
            default: throw new AssertionError("unexpected cold class index");
        }
        try {
            Class.forName("samples.gc.GcThreadsDemo$Cold" + id);
            LOADED.incrementAndGet();
        } catch (ClassNotFoundException error) {
            throw new AssertionError(error);
        }
    }

    private static final class CleanupAction implements Runnable {
        @Override
        public void run() {
            CLEANED.countDown();
        }
    }

    private static void registerCleanablesConcurrently() throws Exception {
        Thread[] registrars = new Thread[WORKERS];
        for (int i = 0; i < registrars.length; i++) {
            registrars[i] = new Thread(() -> {
                for (int j = 0; j < CLEANABLES_PER_WORKER; j++) {
                    CLEANER.register(new Marker(j), new CleanupAction());
                }
            }, "gc-cleaner-registrar-" + i);
            registrars[i].start();
        }
        for (Thread registrar : registrars) {
            registrar.join(30_000);
            if (registrar.isAlive()) {
                throw new AssertionError("cleaner registrar failed to terminate");
            }
        }
        collectWithGarbage();
        if (!CLEANED.await(30, TimeUnit.SECONDS)) {
            throw new AssertionError("only " + CLEANED.getCount() + " cleanables remain");
        }
    }

    private static void raceColdClassLoading() throws Exception {
        Thread[] loaders = new Thread[WORKERS];
        CountDownLatch start = new CountDownLatch(1);
        for (int i = 0; i < loaders.length; i++) {
            final int id = i;
            loaders[i] = new Thread(() -> {
                try {
                    if (!start.await(30, TimeUnit.SECONDS)) {
                        throw new AssertionError("class-loader start was not released");
                    }
                    loadCold(id);
                } catch (Exception error) {
                    throw new AssertionError(error);
                }
            }, "gc-class-loader-" + i);
            loaders[i].start();
        }
        start.countDown();
        if (!CLASS_STARTED.await(30, TimeUnit.SECONDS)) {
            throw new AssertionError("cold class initializer did not reach its rendezvous");
        }
        collectWithGarbage();
        RELEASE_CLASS.countDown();
        for (Thread loader : loaders) {
            loader.join(30_000);
            if (loader.isAlive()) {
                throw new AssertionError("class loader failed to terminate");
            }
        }
        if (LOADED.get() != WORKERS) {
            throw new AssertionError("class-loading startup storm incomplete: " + LOADED.get());
        }
    }

    public static void main(String[] args) throws Exception {
        if (args.length > 0 && args[0].equals("class-loading")) {
            raceColdClassLoading();
            System.out.println("class-loading-ok");
            return;
        }
        if (args.length > 0 && args[0].equals("cleaner")) {
            registerCleanablesConcurrently();
            System.out.println("cleaners-ok");
            return;
        }

        Thread[] workers = new Thread[WORKERS];
        for (int i = 0; i < workers.length; i++) {
            workers[i] = new LocalOnlyWorker(i);
            workers[i].start();
        }
        if (!READY.await(30, TimeUnit.SECONDS)) {
            throw new AssertionError("local-only workers did not reach the blocked state");
        }
        collectWithGarbage();
        RELEASE.countDown();
        for (Thread worker : workers) {
            worker.join(30_000);
            if (worker.isAlive()) {
                throw new AssertionError("local-only worker failed to terminate");
            }
        }
        if (SURVIVED.get() != WORKERS) {
            throw new AssertionError("blocked-thread local roots survived " + SURVIVED.get()
                    + "/" + WORKERS);
        }

        raceColdClassLoading();
        registerCleanablesConcurrently();
        System.out.println("threads-ok");
    }
}
