package samples.javacore.loadlibrary.reflection;

import java.lang.reflect.Constructor;
import java.lang.reflect.Executable;
import java.lang.reflect.Method;

public class JniReflectedMethodsDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native Executable roundTrip(Executable reflected, Class<?> owner, boolean isStatic);

    public static void main(String[] args) throws Exception {
        Method instance = Sample.class.getDeclaredMethod("instanceMethod");
        Method staticMethod = Sample.class.getDeclaredMethod("staticMethod");
        Constructor<Sample> constructor = Sample.class.getDeclaredConstructor();
        System.out.println("instance=" + instance.equals(roundTrip(instance, Sample.class, false)));
        System.out.println("static=" + staticMethod.equals(roundTrip(staticMethod, Sample.class, true)));
        System.out.println("constructor=" + constructor.equals(roundTrip(constructor, Sample.class, false)));
    }
}

class Sample {
    Sample() {}

    void instanceMethod() {}

    static void staticMethod() {}
}
