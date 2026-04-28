package samples.javacore.loadlibrary.example;

public class JniClassOperationsDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native Class<?> findClass(String className);

    public static void main(String[] args) {
        // Positive test cases for various class name formats
        test("normal String", "java/lang/String");
        test("object array", "[Ljava/lang/String;");
        test("primitive array", "[I");
        test("multi-dim array", "[[Ljava/lang/String;");

        // Negative test cases for various invalid formats
        //test("dot notation", "java.lang.String"); // fixme: should not pass https://github.com/hextriclosan/rusty-jvm/issues/810
        //test("primitive I", "I"); // fixme: should not pass https://github.com/hextriclosan/rusty-jvm/issues/810
        //test("primitive J", "J"); // fixme: should not pass https://github.com/hextriclosan/rusty-jvm/issues/810
        //test("primitive V", "V"); // fixme: should not pass https://github.com/hextriclosan/rusty-jvm/issues/810

        test("primitive int", "int");
        test("primitive long", "long");
        test("primitive void", "void");
        test("non-existing", "this/class/DoesNotExist");
        test("broken descriptor", "Ljava/lang/String");
    }

    private static void test(String label, String className) {
        try {
            Class<?> clazz = findClass(className);
            System.out.printf("[OK] %s -> %s%n", label, clazz);
        } catch (Throwable t) {
            System.out.printf("[FAIL] %s -> %s%n", label, t);
        }
    }
}
