package samples.reflection.trivial;

public class ArrayClass {
    public static void main(String[] args) {
        testArrayType(int[][][].class);
        testArrayType(String[][].class);
        testArrayType(Runnable[][].class);
    }

    private static void testArrayType(Class<?> clazz) {
        System.out.print("Analyzing class: ");
        System.out.println(clazz.getName());
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
        System.out.print("Class: ");
        System.out.println(clazz);

        System.out.print("  isPrimitive: ");
        System.out.println(clazz.isPrimitive());

        System.out.print("  isArray: ");
        System.out.println(clazz.isArray());
    }
}
