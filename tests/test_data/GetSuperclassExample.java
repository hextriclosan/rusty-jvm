package samples.reflection.trivial.classgetsuperclassexample;

import java.util.HashMap;
import java.util.concurrent.TimeUnit;

public class GetSuperclassExample {
    public static void main(String[] args) {
        // 1. Basic Case: Standard class hierarchy
        printSuperclass(String.class); // java.lang.Object
        printSuperclass(Integer.class); // java.lang.Number

        // 2. Case: Object has no superclass
        printSuperclass(Object.class); // null

        // 3. Case: Interfaces have no superclass
        printSuperclass(Runnable.class); // null

        // 4. Case: Interface extending another interface
        printSuperclass(ExtendedRunnable.class); // null

        // 5. Case: Primitive types have no superclass
        printSuperclass(int.class); // null
        printSuperclass(void.class); // null

        // 6. Case: Arrays
        printSuperclass(String[].class); // java.lang.Object
        printSuperclass(int[].class); // java.lang.Object

        // 7. Case: Array of interfaces
        printSuperclass(Runnable[].class); // java.lang.Object

        // 8. Case: Anonymous class
        Object anonymous = new Object() {};
        printSuperclass(anonymous.getClass()); // java.lang.Object

        // 9. Case: Local class
        class LocalClass {}
        printSuperclass(LocalClass.class); // java.lang.Object

        // 10. Case: Nested/Inner class
        printSuperclass(InnerClass.class); // java.lang.Object

        // 11. Case: Enum classes
        printSuperclass(TimeUnit.class); // java.lang.Enum

        // 12. Case: Classes extending from abstract classes
        printSuperclass(HashMap.class); // AbstractClass
    }

    private static void printSuperclass(Class<?> clazz) {
        System.out.println("Superclass of " + clazz.getName() + ": " + clazz.getSuperclass());
    }

    // Inner class
    static class InnerClass {}

    // Interface extending another interface
    interface ExtendedRunnable extends Runnable {}
}
