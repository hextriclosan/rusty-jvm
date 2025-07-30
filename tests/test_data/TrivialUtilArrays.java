package samples.javabase.util.arrays.trivial;

import java.util.Arrays;

public class TrivialUtilArrays {
    public static void main(String[] args) {
        int[] first = {10, 20, 30, 40, 50, 60, 70, 80, 90, 100};
        int[] second = {10, 20, 30, 40, 50, 60, 70, 80, 90, -100};
        print("Binary search result: ", Arrays.binarySearch(first, 100));
        print("Arrays are equal: ", Arrays.equals(first, second));
        print("Arrays compare: ", Arrays.compare(first, second));
        print("Arrays compareUnsigned: ", Arrays.compareUnsigned(first, second));
        print("Mismatched index: ", Arrays.mismatch(first, second));
        print("Hash code: ", Arrays.hashCode(first));
        print("List: ", Arrays.asList(1, 2, 3));
        print("String: ", Arrays.toString(first));
        print("Copied: ", Arrays.copyOf(first, 5));
        print("Copied of range: ", Arrays.copyOfRange(first, 5, 10));
        Arrays.fill(first, 1, 9, 42);
        print("Filled: ", first);
        // Arrays.sort(first); // invokedynamic
        // Arrays.parallelSort(first); Native method java/lang/Class:getDeclaredFields0:(Z)[Ljava/lang/reflect/Field; not found
        Object[] first2d = new int[][]{{1, 2}, {3, 4}};
        Object[] second2d = new int[][]{{1, 2}, {3, 4}};
        print("Arrays are deep equals: ", Arrays.deepEquals(first2d, second2d));
        print("Deep hash code: ", Arrays.deepHashCode(first2d));
        print("Deep string: ", Arrays.deepToString(first2d));
    }

    private static void print(String msg, Object value) {
        System.out.println(msg + value);
    }

    private static void print(String msg, int[] arr) {
        print(msg, Arrays.toString(arr));
    }
}
