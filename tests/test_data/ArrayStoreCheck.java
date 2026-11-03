    package samples.arrays.store;

import java.io.Serializable;
import java.util.Arrays;

public class ArrayStoreCheck {
    public static void main(String[] args) {
        run("same class", new String[2], 0, "value");
        run("subclass", new Number[2], 1, 42);
        run("null", new String[] {"old", "old"}, 0, null);
        run("interface", new Runnable[2], 0, new Task());
        run("primitive array as Object", new Object[2], 0, new int[] {1});
        run("array as Cloneable", new Cloneable[2], 0, new int[] {1});
        run("array as Serializable", new Serializable[2], 0, new String[] {"a"});
        run("nested array", new Object[2][], 0, new String[] {"a"});
        run("covariant nested array", new Number[2][], 0, new Integer[] {7});

        run("unrelated class", new String[] {"old", "old"}, 1, 42);
        run("not an interface implementor", new Runnable[2], 0, "value");
        run("superclass into subclass array", new Integer[2], 0, new Object());
        run("primitive array as Object[]", new Object[2][], 1, new int[] {1});
        run("object into primitive arrays", new int[2][], 0, "value");
        run("object into nested arrays", new String[2][][], 0, "value");
        run("array of wrong type", new Integer[2][], 0, new Number[] {7});
        run("out of bounds before type check", new String[2], 2, 42);
        run("null array before type check", null, 0, 42);
    }

    private static void store(Object[] array, int index, Object value) {
        array[index] = value;
    }

    private static class Task implements Runnable {
        @Override
        public void run() {
        }

        @Override
        public String toString() {
            return "task";
        }
    }

    private static void run(String label, Object[] array, int index, Object value) {
        try {
            store(array, index, value);
            System.out.println(label + " ok " + Arrays.deepToString(array));
        } catch (RuntimeException exception) {
            System.out.println(label + " -> " + exception.getClass().getName() + ": " + exception.getMessage()
                    + " " + Arrays.deepToString(array));
        }
    }
}
