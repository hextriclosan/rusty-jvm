// javac -XDstringConcat=inline -d . MethodHandleStaticInitCases.java
package samples.staticinit.bymethodhandle;

import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;

public class MethodHandleStaticInitCases {
    public static void main(String[] args) throws Throwable {
        System.out.println("Main method started");
        MethodHandles.Lookup lookup = MethodHandles.lookup();

        System.out.println("=== Case 1: Invoking a static method (triggers initialization) ===");
        MethodHandle staticMethodHandle = lookup.findStatic(Class1.class, "staticMethod", MethodType.methodType(void.class));
        System.out.println("MethodHandle obtained, but class not initialized yet");
        staticMethodHandle.invokeExact(); // Triggers initialization
        System.out.println();

        System.out.println("=== Case 2: Invoking a constructor (triggers initialization) ===");
        MethodHandle constructorHandle = lookup.findConstructor(Class2.class, MethodType.methodType(void.class));
        System.out.println("Constructor MethodHandle obtained, but class not initialized yet");
        Class2 instance = (Class2) constructorHandle.invokeExact(); // Triggers initialization
        System.out.println();

        System.out.println("=== Case 3: Accessing a static field getter (triggers initialization) ===");
        MethodHandle staticFieldGetter = lookup.findStaticGetter(Class3.class, "STATIC_FIELD", int.class);
        System.out.println("Static field MethodHandle obtained, but class not initialized yet");
        int value = (int) staticFieldGetter.invokeExact(); // Triggers initialization
        System.out.println("Static field value: " + value);
        System.out.println();

        System.out.println("=== Case 4: Accessing a static field setter (triggers initialization) ===");
        MethodHandle staticFieldSetter = lookup.findStaticSetter(Class4.class, "STATIC_FIELD", int.class);
        System.out.println("Static field setter MethodHandle obtained, but class not initialized yet");
        staticFieldSetter.invokeExact(100); // triggers initialization then sets new value,
        System.out.println("Updated STATIC_FIELD: " + Class4.STATIC_FIELD);
        System.out.println();

        System.out.println("Main method complete");
    }
}

class Class1 {
    static {
        System.out.println("Static block executed: Class1 initialized");
    }

    public static void staticMethod() {
        System.out.println("Class1: Static method invoked");
    }
}

class Class2 {
    static {
        System.out.println("Static block executed: Class2 initialized");
    }

    public Class2() {
        System.out.println("Class2: Constructor executed");
    }
}

class Class3 {
    static {
        System.out.println("Static block executed: Class3 initialized");
    }

    static int STATIC_FIELD = 42;
}

class Class4 {
    static int STATIC_FIELD = 10;

    static {
        System.out.println("Static block executed: Class4 initialized");
        System.out.println("Initial STATIC_FIELD value: " + STATIC_FIELD);
    }
}
