package samples.javacore.loadlibrary.objectops;

public class JniAllocObjectDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native Object allocate(Class<?> target);

    public static void main(String[] args) {
        Sample sample = (Sample) allocate(Sample.class);
        System.out.println("value=" + sample.value + " inherited=" + sample.inherited + " name=" + sample.name);
        System.out.println("constructor calls=" + Base.constructorCalls);

        System.out.println("initialized before=" + InitProbe.initialized);
        allocate(Lazy.class);
        System.out.println("initialized after=" + InitProbe.initialized);

        report("abstract", AbstractSample.class);
        report("interface", Runnable.class);
        report("primitive", int.class);
        report("array", int[].class);
        report("Class", Class.class);
        report("null", null);
    }

    private static void report(String label, Class<?> target) {
        Object result = allocate(target);
        String description = result instanceof Throwable t
                ? t.getClass().getName() + ": " + t.getMessage()
                : String.valueOf(result);
        System.out.println(label + " -> " + description);
    }
}

class Base {
    static int constructorCalls;
    int inherited;

    Base() {
        constructorCalls++;
        inherited = 7;
    }
}

class Sample extends Base {
    int value;
    String name;

    Sample() {
        value = 42;
        name = "set";
    }
}

class Lazy {
    static {
        InitProbe.initialized = true;
    }
}

class InitProbe {
    static boolean initialized;
}

abstract class AbstractSample {
}
