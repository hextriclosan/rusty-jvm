package samples.gc;

import java.lang.ref.WeakReference;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;

public final class GcJniDemo {
    private GcJniDemo() {}

    private static final class DerivedClassNotFoundException extends ClassNotFoundException {}

    public static final class TestSystemClassLoader extends ClassLoader {
        public TestSystemClassLoader(ClassLoader parent) {
            super(parent);
        }

        @Override
        protected Class<?> loadClass(String name, boolean resolve) throws ClassNotFoundException {
            if (name.equals("samples.gc.GcJniDemo$MissingForJniFindClass")) {
                throw new DerivedClassNotFoundException();
            }
            return super.loadClass(name, resolve);
        }
    }

    private static native boolean aliasesAndPromotes(Object value);
    private static native Object nestedFrameEscape(Object value);
    private static native Object returnGlobal(Object value);
    private static native void deleteGlobal(Object value);
    private static native Object returnWeak(Object value);
    private static native void deleteWeak(Object value);
    private static native String createString();
    private static native Class<?> findMissingClass();
    private static native boolean stringLeaseAcrossGc(String value);
    private static native void invokeCallback(Callback callback);
    private static native boolean holdCopiedBufferAcrossGc(int[] values, CountDownLatch ready);
    private static native void releaseHeldBuffer();
    private static native boolean exerciseReleaseModes(int[] values);

    private interface Callback {
        void run();
    }

    private static void require(boolean condition, String message) {
        if (!condition) {
            throw new AssertionError(message);
        }
    }

    private static void proveBufferLeaseAcrossCollection() throws InterruptedException {
        int[] values = {1, 2, 3};
        CountDownLatch ready = new CountDownLatch(1);
        Thread collector = new Thread(() -> {
            try {
                if (!ready.await(30, TimeUnit.SECONDS)) {
                    throw new AssertionError("native buffer lease was not published");
                }
                System.gc();
                releaseHeldBuffer();
            } catch (InterruptedException error) {
                throw new AssertionError(error);
            }
        }, "jni-gc-lease-collector");
        collector.start();
        require(holdCopiedBufferAcrossGc(values, ready), "native buffer holder failed");
        collector.join(30_000);
        require(!collector.isAlive(), "collector did not release the copied buffer");
        require(values[0] == 77, "copied buffer did not copy back after collection");
        System.out.println("lease-gc-ok");
    }

    private static void proveReleasedBufferDoesNotRootArray() {
        int[] values = {1, 2};
        WeakReference<int[]> weak = new WeakReference<>(values);
        require(exerciseReleaseModes(values), "array release-mode exercise failed");
        require(values[0] == 30 && values[1] == 2, "JNI release modes copied wrong contents");
        values = null;
        System.gc();
        require(weak.get() == null, "released JNI buffers kept the array alive");
        System.out.println("release-modes-ok");
    }

    public static void main(String[] args) throws Exception {
        System.loadLibrary("jni_test_lib");
        Object value = new Object();
        require(aliasesAndPromotes(value), "JNI aliases or weak promotion failed");
        require(nestedFrameEscape(value) == value, "nested local-frame escape failed");

        Object global = returnGlobal(value);
        System.gc();
        require(global == value, "direct global return did not survive collection");
        deleteGlobal(global);

        Object weak = returnWeak(value);
        System.gc();
        require(weak == value, "direct weak return or weak promotion failed");
        deleteWeak(weak);
        require("native-created-across-gc".equals(createString()),
                "native-created object return failed");

        boolean callbackCaught = false;
        try {
            invokeCallback(() -> {
                throw new IllegalStateException("callback-exception");
            });
        } catch (IllegalStateException expected) {
            callbackCaught = "callback-exception".equals(expected.getMessage());
        }
        require(callbackCaught, "callback exception was not preserved");
        boolean subclassConverted = false;
        try {
            findMissingClass();
        } catch (NoClassDefFoundError expected) {
            subclassConverted =
                    "samples/gc/GcJniDemo$MissingForJniFindClass".equals(expected.getMessage());
        }
        require(subclassConverted, "FindClass did not map the CNFE subclass with its internal name");
        System.out.println("find-class-subclass-ok");
        System.out.println("references-ok");
        require(stringLeaseAcrossGc("leased-string-across-gc"),
                "copied string buffer did not survive collection");
        System.out.println("string-lease-ok");

        proveBufferLeaseAcrossCollection();
        proveReleasedBufferDoesNotRootArray();
        System.out.println("jni-gc-ok");
    }
}
