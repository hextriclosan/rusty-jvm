package samples.javacore.loadlibrary.objectops;

public class JniAllocObjectDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native Sample allocate(Class<?> target);

    public static void main(String[] args) {
        Sample.constructorCalls = 0;
        Sample sample = allocate(Sample.class);
        System.out.println("value=" + sample.value);
        System.out.println("constructor calls=" + Sample.constructorCalls);
    }
}

class Sample {
    static int constructorCalls;
    int value;

    Sample() {
        constructorCalls++;
        value = 42;
    }
}
