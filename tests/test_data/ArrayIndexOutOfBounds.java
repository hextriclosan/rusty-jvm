package samples.arrays.bounds;

public class ArrayIndexOutOfBounds {
    private static int intSink;
    private static Object objectSink;

    public static void main(String[] args) {
        runCase("primitive load negative", 0);
        runCase("reference load at length", 1);
        runCase("primitive store at length", 2);
        runCase("reference store negative", 3);
        runCase("null store", 4);
    }

    private static void runCase(String label, int operation) {
        try {
            switch (operation) {
                case 0 -> {
                    int[] array = new int[1];
                    intSink = array[-1];
                }
                case 1 -> {
                    Object[] array = new Object[1];
                    objectSink = array[1];
                }
                case 2 -> {
                    int[] array = new int[1];
                    array[1] = 42;
                }
                case 3 -> {
                    Object[] array = new Object[1];
                    array[-1] = new Object();
                }
                case 4 -> {
                    int[] array = null;
                    array[0] = 42;
                }
                default -> throw new IllegalArgumentException();
            }
            System.out.println(label + ": no exception");
        } catch (ArrayIndexOutOfBoundsException exception) {
            System.out.println(label + ": " + exception.getClass().getName());
        } catch (NullPointerException exception) {
            System.out.println(label + ": " + exception.getClass().getName());
        }
    }
}
