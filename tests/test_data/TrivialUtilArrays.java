package samples.javabase.util.arrays.trivial;

import java.util.Arrays;

public class TrivialUtilArrays {
    public static void main(String[] args) {
        int[] arr = new int[] {10, 20, 30, 40, 50, 60, 70, 80, 90, 100};
        int result =  Arrays.binarySearch(arr, 100);
        System.out.println(result);
    }
}
