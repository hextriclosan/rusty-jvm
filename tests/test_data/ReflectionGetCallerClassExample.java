// javac --add-exports java.base/jdk.internal.reflect=ALL-UNNAMED  -d . ReflectionGetCallerClassExample.java
package samples.jdkinternal.reflection.getcallerclass;

import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodHandles.Lookup;
import java.lang.invoke.MethodType;
import java.lang.reflect.Method;
import java.util.function.Supplier;
import jdk.internal.reflect.CallerSensitive;
import jdk.internal.reflect.Reflection;

public class ReflectionGetCallerClassExample {
    public static void main(String[] args) throws Throwable {
        // MethodHandles.lookup() is caller-sensitive: its lookup class is the class that called it
        report("direct", MethodHandles.lookup());
        report("from another class", Caller.lookup());
        report("from a nested class", Nested.lookup());
        report("from a lambda", ((Supplier<Lookup>) () -> MethodHandles.lookup()).get());
        report("from a deeper frame of the same class", Caller.lookupFromDeeperFrame());

        // Method.invoke and the reflection machinery below it are not callers
        Method lookup = MethodHandles.class.getMethod("lookup");
        report("caller-sensitive method through reflection", (Lookup) lookup.invoke(null));
        Method callerLookup = Caller.class.getDeclaredMethod("lookup");
        report("its caller through reflection", (Lookup) callerLookup.invoke(null));

        // nor is the method handle machinery
        Lookup viaHandle = (Lookup) MethodHandles.lookup()
                .findStatic(MethodHandles.class, "lookup", MethodType.methodType(Lookup.class))
                .invoke();
        report("caller-sensitive method through a method handle", viaHandle);
        Lookup callerViaHandle = (Lookup) MethodHandles.lookup()
                .findStatic(Caller.class, "lookup", MethodType.methodType(Lookup.class))
                .invokeExact();
        report("its caller through a method handle", callerViaHandle);

        // getCallerClass needs a caller-sensitive caller, and only the JDK's methods are
        reportError("called from a method that is not caller-sensitive", Caller::notCallerSensitive);
        reportError("called from an annotated method outside the JDK", Caller::annotatedOutsideJdk);
    }

    private static void report(String how, Lookup lookup) {
        System.out.println(how + ": " + lookup.lookupClass().getName());
    }

    private static void reportError(String how, ThrowingRunnable action) {
        try {
            action.run();
            System.out.println(how + ": no error");
        } catch (Throwable e) {
            System.out.println(how + ": " + e);
        }
    }

    interface ThrowingRunnable {
        void run() throws Throwable;
    }

    static class Nested {
        static Lookup lookup() {
            return MethodHandles.lookup();
        }
    }
}

class Caller {
    static Lookup lookup() {
        return MethodHandles.lookup();
    }

    static Lookup lookupFromDeeperFrame() {
        return intermediate();
    }

    private static Lookup intermediate() {
        return MethodHandles.lookup();
    }

    static void notCallerSensitive() {
        Reflection.getCallerClass();
    }

    @CallerSensitive
    static void annotatedOutsideJdk() {
        Reflection.getCallerClass();
    }
}
