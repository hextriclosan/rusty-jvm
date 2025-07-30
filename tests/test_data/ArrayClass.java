package samples.reflection.trivial;

public class ArrayClass {
    public static void main(String[] args) {
        testArrayType(int[][][].class);
        testArrayType(String[][].class);
        testArrayType(Runnable[][].class);
    }

    private static void testArrayType(Class<?> clazz) {
        System.out.println("Analyzing class: " + clazz.getName());
        System.out.println("----------------------------------------");

        while (clazz != null) {
            printClassDetails(clazz);
            clazz = clazz.getComponentType();
        }

        System.out.println("End of analysis.");
        System.out.println("========================================");
        System.out.println();
    }

    private static void printClassDetails(Class<?> clazz) {
        System.out.println("Class: " + clazz);
        System.out.println("  isPrimitive: " + clazz.isPrimitive());
        System.out.println("  isArray: " + clazz.isArray());
    }
}
