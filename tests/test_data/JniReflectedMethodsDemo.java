package samples.javacore.loadlibrary.reflection;

import java.lang.reflect.Constructor;
import java.lang.reflect.Executable;
import java.lang.reflect.Method;

public class JniReflectedMethodsDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native Executable roundTrip(Executable reflected, Class<?> owner, boolean isStatic);

    private static native boolean sameIdAsLookup(Executable reflected, Class<?> lookupClass, String name, String signature, boolean isStatic);

    private static native int invokeInstance(Executable reflected, Object target);

    private static native int invokeStatic(Executable reflected, Class<?> targetClass);

    public static void main(String[] args) throws Exception {
        Method instance = Sample.class.getDeclaredMethod("instanceMethod");
        Method staticMethod = Sample.class.getDeclaredMethod("staticMethod");
        Method inherited = Sample.class.getDeclaredMethod("inheritedMethod");
        Constructor<Sample> constructor = Sample.class.getDeclaredConstructor();
        System.out.println("instance=" + instance.equals(roundTrip(instance, Sample.class, false)));
        System.out.println("static=" + staticMethod.equals(roundTrip(staticMethod, Sample.class, true)));
        System.out.println("constructor=" + constructor.equals(roundTrip(constructor, Sample.class, false)));

        System.out.println("instance id=" + sameIdAsLookup(instance, Sample.class, "instanceMethod", "()I", false));
        System.out.println("static id=" + sameIdAsLookup(staticMethod, Sample.class, "staticMethod", "()I", true));
        System.out.println("constructor id=" + sameIdAsLookup(constructor, Sample.class, "<init>", "()V", false));
        System.out.println("inherited id=" + sameIdAsLookup(inherited, SubSample.class, "inheritedMethod", "()I", false));

        System.out.println("invoke instance=" + invokeInstance(instance, new Sample()));
        System.out.println("invoke overridden=" + invokeInstance(instance, new SubSample()));
        System.out.println("invoke static=" + invokeStatic(staticMethod, Sample.class));
    }
}

class Sample {
    Sample() {}

    int instanceMethod() {
        return 1;
    }

    static int staticMethod() {
        return 2;
    }

    int inheritedMethod() {
        return 4;
    }
}

class SubSample extends Sample {
    @Override
    int instanceMethod() {
        return 3;
    }
}
