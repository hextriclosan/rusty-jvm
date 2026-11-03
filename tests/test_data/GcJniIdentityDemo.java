package samples.gc;

public class GcJniIdentityDemo {
    static { System.loadLibrary("jni_test_lib"); }

    private static native Object aliases(Object object);
    private static native boolean releaseGlobal(Object object);
    private static native Object frames(Object object);
    private static native boolean staleLocal();
    private static native Object capacity(Object object, boolean push);
    private static native boolean buffers(int[] array, String text);
    private static native void callback();

    private static void fail() {
        throw new IllegalStateException("callback");
    }

    public static void main(String[] args) {
        Object object = new Object();
        if (aliases(object) != object || !releaseGlobal(object)) {
            throw new AssertionError("independent JNI aliases or global return");
        }
        System.out.println("aliases-ok");
        if (frames(object) != object || !staleLocal()) {
            throw new AssertionError("JNI frame lifetime");
        }
        System.out.println("frames-ok");
        if (!(capacity(object, true) instanceof OutOfMemoryError)
                || !(capacity(object, false) instanceof OutOfMemoryError)) {
            throw new AssertionError("JNI capacity failure");
        }
        System.out.println("capacity-ok");
        int[] array = {1, 2};
        if (!buffers(array, "A\u0000\uD834\uDD1E") || array[0] != 41 || array[1] != 42) {
            throw new AssertionError("JNI copied buffers");
        }
        System.out.println("buffers-ok");
        try {
            callback();
            throw new AssertionError("missing callback exception");
        } catch (IllegalStateException expected) {
            if (!"callback".equals(expected.getMessage())) {
                throw new AssertionError("wrong callback exception");
            }
        }
        if (aliases(object) != object || !releaseGlobal(object)) {
            throw new AssertionError("JNI recovery after exception");
        }
        System.out.println("callbacks-ok");
    }
}
