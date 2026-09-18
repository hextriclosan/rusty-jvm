package samples.jni.staticfields;

class StaticFieldParent {
    static int value = 17;
}

class StaticFieldChild extends StaticFieldParent {
    static long value = 9_000_000_000L;
}

public class JniStaticFieldResolution {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native int readInt(Class<?> target);
    private static native long readLong(Class<?> target);

    public static void main(String[] args) {
        System.out.println(readInt(StaticFieldChild.class));
        System.out.println(readLong(StaticFieldChild.class));
    }
}
