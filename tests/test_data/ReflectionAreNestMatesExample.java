// javac --add-exports java.base/jdk.internal.reflect=ALL-UNNAMED -d . ReflectionAreNestMatesExample.java
// java --add-exports java.base/jdk.internal.reflect=ALL-UNNAMED samples.reflection.trivial.arenestmatesexample.ReflectionAreNestMatesExample
package samples.reflection.trivial.arenestmatesexample;

import jdk.internal.reflect.Reflection;

public class ReflectionAreNestMatesExample {
    public static void main(String[] args) {
        Class<?> thisClass = ReflectionAreNestMatesExample.class;
        print(thisClass, ReflectionAreNestMatesExample.class);
        print(thisClass, NestedClass.class);
        print(thisClass, InnerClass.class);
        print(NestedClass.class, InnerClass.class);

        class LocalClass {
        }
        print(thisClass, LocalClass.class);

        Class<?> runnable = new Runnable() {
            @Override
            public void run() {}
        }.getClass();
        print(thisClass, runnable);

        Class<?> innerInterface = new InnerInterface() {
        }.getClass();
        print(thisClass, innerInterface);
        print(runnable, innerInterface);

//         var lambda1 = (Runnable) () -> {}; // Execution Error: Unknown reference opcode: 186
//         print(thisClass, lambda1.getClass());
//         var lambda2 = (Runnable) () -> {};
//         print(thisClass, lambda2.getClass());

        print(thisClass, String.class);
        print(thisClass, ReflectionAreNestMatesExample[].class);
        // print(thisClass, int.class); // crashes the JVM :-)
    }

    static class NestedClass {
    }

    class InnerClass {
    }

    interface InnerInterface {
    }

    private static void print(Class<?> thisClass, Class<?> classToCheck) {
        System.out.printf("%s and %s are nest mates: %b%n", thisClass, classToCheck, Reflection.areNestMates(thisClass, classToCheck));
    }
}
