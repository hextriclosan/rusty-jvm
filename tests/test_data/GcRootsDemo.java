package samples.gc;

import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.reflect.Method;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;

public final class GcRootsDemo {
    private static Object staticRoot;
    private static int failedAttempts;

    private static final class Marker {
        private final int value;

        Marker(int value) {
            this.value = value;
        }
    }

    private static final class Holder {
        Object child;
    }

    private static final class RecursiveInit {
        static final int value;

        static {
            value = initializeRecursively();
        }

        private static synchronized int initializeRecursively() {
            return value == 0 ? 23 : -1;
        }
    }

    private static final class FailedInit {
        static {
            failedAttempts++;
            if (failedAttempts > 0) {
                throw new IllegalStateException("expected initialization failure");
            }
        }
    }

    private static synchronized void failInsideSynchronizedMethod() {
        throw new IllegalStateException("expected synchronized exit");
    }

    private static synchronized void verifySynchronizedMonitorReleased(
            CountDownLatch acquired) {
        acquired.countDown();
    }

    private static synchronized native boolean nativeOuter(Object value, int marker);

    private static native boolean nativeInner(Object value, int marker);

    private static void nativeCallback(Object value, int marker) {
        if (!Thread.holdsLock(GcRootsDemo.class)
                || !nativeInner(value, marker)
                || ((Marker) value).value != marker) {
            throw new AssertionError("innermost native lost its callback reference");
        }
    }

    private static void collectFromNative() {
        collectWithGarbage();
    }

    private static Object mixedTarget(long primitive, Object reference) {
        if (primitive != 0x1020304050607080L) {
            throw new AssertionError("primitive argument was corrupted");
        }
        collectWithGarbage();
        return reference;
    }

    private static void collectWithGarbage() {
        byte[] unreachable = new byte[4096];
        unreachable[0] = 1;
        unreachable = null;
        System.gc();
    }

    public static void main(String[] args) throws Throwable {
        System.loadLibrary("jni_test_lib");
        testStaticRoot();
        testInstanceFieldRoot();
        testReferenceArrayRoot();
        testMixedPrimitiveReferencePath();
        testPartialMultiArray();
        testNestedNativeCallbacks();
        testReflectionMetadataRoot();
        testClassInitialization();
        testSynchronizedExceptionExit();
        testIdentityAndMonitorLifetime();
        System.out.println("roots-ok");
    }

    private static void testStaticRoot() {
        staticRoot = new Marker(11);
        collectWithGarbage();
        require(((Marker) staticRoot).value == 11, "static root");
        staticRoot = null;
    }

    private static void testInstanceFieldRoot() {
        Holder holder = new Holder();
        holder.child = new Marker(13);
        collectWithGarbage();
        require(((Marker) holder.child).value == 13, "instance-field root");
    }

    private static void testReferenceArrayRoot() {
        Object[] references = {new Marker(17)};
        collectWithGarbage();
        require(((Marker) references[0]).value == 17, "reference-array root");
    }

    private static void testMixedPrimitiveReferencePath() throws Throwable {
        MethodHandle mixed = MethodHandles.lookup().findStatic(
                GcRootsDemo.class,
                "mixedTarget",
                MethodType.methodType(Object.class, long.class, Object.class));
        Object mixedValue = mixed.invokeExact(0x1020304050607080L, (Object) new Marker(19));
        require(((Marker) mixedValue).value == 19, "mixed primitive/reference path");
    }

    private static void testPartialMultiArray() {
        Object[][][] partial = new Object[3][][];
        partial[0] = new Object[2][];
        partial[0][1] = new Object[] {new Marker(29)};
        collectWithGarbage();
        require(((Marker) partial[0][1][0]).value == 29, "partial multi-array");
        require(partial[1] == null && partial[0][0] == null, "partial array null slots");
    }

    private static void testNestedNativeCallbacks() throws InterruptedException {
        Object callbackValue = new Marker(31);
        require(nativeOuter(callbackValue, 31), "nested native callbacks");
        CountDownLatch acquired = new CountDownLatch(1);
        Thread probe = new Thread(() -> verifySynchronizedMonitorReleased(acquired));
        probe.start();
        require(acquired.await(5, TimeUnit.SECONDS),
                "synchronized native method retained its monitor after returning");
        probe.join(5000);
        require(!probe.isAlive(), "native monitor probe did not terminate");
    }

    private static void testReflectionMetadataRoot() throws Exception {
        Class<?> metadata = GcRootsDemo.class;
        Method[] methods = metadata.getDeclaredMethods();
        int methodCount = methods.length;
        methods = null;
        collectWithGarbage();
        require(methodCount > 0 && metadata.getDeclaredMethod("mixedTarget", long.class,
                Object.class) != null, "reflection metadata roots");
    }

    private static void testClassInitialization() throws Exception {
        require(RecursiveInit.value == 23, "class-init owner recursion");
        boolean firstFailed = false;
        try {
            Class.forName("samples.gc.GcRootsDemo$FailedInit");
        } catch (ExceptionInInitializerError expected) {
            firstFailed = true;
        }
        boolean retryFailed = false;
        try {
            Class.forName("samples.gc.GcRootsDemo$FailedInit");
        } catch (NoClassDefFoundError expected) {
            retryFailed = true;
        }
        require(firstFailed && retryFailed && failedAttempts == 1,
                "failed class initialization must stay failed");
    }

    private static void testSynchronizedExceptionExit() throws InterruptedException {
        try {
            failInsideSynchronizedMethod();
            throw new AssertionError("synchronized method did not throw");
        } catch (IllegalStateException expected) {
            // The synchronized method must release the Class monitor on abrupt completion.
        }
        CountDownLatch acquired = new CountDownLatch(1);
        Thread probe = new Thread(() -> verifySynchronizedMonitorReleased(acquired));
        probe.start();
        require(acquired.await(5, TimeUnit.SECONDS),
                "synchronized method retained its monitor after throwing");
        probe.join(5000);
        require(!probe.isAlive(), "synchronized monitor probe did not terminate");
    }

    private static void testIdentityAndMonitorLifetime() {
        Object identity = new Marker(37);
        int identityHash = System.identityHashCode(identity);
        for (int cycle = 0; cycle < 4; cycle++) {
            collectWithGarbage();
            require(System.identityHashCode(identity) == identityHash,
                    "identity hash changed across collections");
            require(((Marker) identity).value == 37, "identity root did not survive");
        }

        synchronized (identity) {
            collectWithGarbage();
            require(Thread.holdsLock(identity), "active monitor was lost");
        }
    }

    private static void require(boolean condition, String message) {
        if (!condition) {
            throw new AssertionError(message);
        }
    }
}
