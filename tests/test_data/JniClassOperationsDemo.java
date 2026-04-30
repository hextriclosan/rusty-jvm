package samples.javacore.loadlibrary.example;

import java.util.HashMap;

public class JniClassOperationsDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native Class<?> findClass(String className);
    private static native Class<?> getSuperclass(Class<?> clazz);

    public static void main(String[] args) {
        findClassDemo();
        getSuperclassDemo();
    }

    private static void findClassDemo() {
        System.out.println("=== Testing FindClass with various class name formats ===");

        // Positive test cases for various class name formats
        testFindClass("normal String", "java/lang/String");
        testFindClass("object array", "[Ljava/lang/String;");
        testFindClass("primitive array", "[I");
        testFindClass("multi-dim array", "[[Ljava/lang/String;");

        // Negative test cases for various invalid formats
        //testFindClass("dot notation", "java.lang.String"); // fixme: should not pass https://github.com/hextriclosan/rusty-jvm/issues/810
        //testFindClass("primitive I", "I"); // fixme: should not pass https://github.com/hextriclosan/rusty-jvm/issues/810
        //testFindClass("primitive J", "J"); // fixme: should not pass https://github.com/hextriclosan/rusty-jvm/issues/810
        //testFindClass("primitive V", "V"); // fixme: should not pass https://github.com/hextriclosan/rusty-jvm/issues/810

        testFindClass("primitive int", "int");
        testFindClass("primitive long", "long");
        testFindClass("primitive void", "void");
        testFindClass("non-existing", "this/class/DoesNotExist");
        testFindClass("broken descriptor", "Ljava/lang/String");
    }

    private static void testFindClass(String label, String className) {
        try {
            Class<?> clazz = findClass(className);
            System.out.printf("[OK] %s -> %s%n", label, clazz);
        } catch (Throwable t) {
            System.out.printf("[FAIL] %s -> %s%n", label, t);
        }
    }

    private static void getSuperclassDemo() {
        System.out.println("=== Testing GetSuperclass with various class name formats ===");

        testGetSuperclass(String.class);
        testGetSuperclass(HashMap.class);
        testGetSuperclass(Runnable.class);
        testGetSuperclass(Object.class);
        testGetSuperclass(int.class);
        testGetSuperclass(byte[][].class);
    }

    private static void testGetSuperclass(Class<?> clazz) {
        Class<?> superClazz = getSuperclass(clazz);
        System.out.printf("Super class of %s is %s%n", clazz, superClazz);
    }
}
