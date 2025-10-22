package samples.arrays.tostringexample;

import java.util.Arrays;

public class ArrayToStringExample {
    public static void main(String[] args) {
        byte[] bytes = {-128, -1, 0, 1, 127};
        System.out.println(Arrays.toString(bytes));

        boolean[] booleans = {true, false, true, true, false};
        System.out.println(Arrays.toString(booleans));

        short[] shorts = {-17000, -2000, 0, 2000, 17000};
        System.out.println(Arrays.toString(shorts));

        char[] chars = {'A', 'r', 'r', 'a', 'y', 'Ї', '⅒'};
        System.out.println(Arrays.toString(chars));

        int[] ints = {-2000_000_000, -1, 0, 1, 2000_000_000};
        System.out.println(Arrays.toString(ints));

        long[] longs = {-9_000_000_000_000_000_000L, -1L, 0L, 1L, 9_000_000_000_000_000_000L};
        System.out.println(Arrays.toString(longs));

        float[] floats = {3.14f, 2.71f, 1.41f, 1.73f};
        System.out.println(Arrays.toString(floats));

        double[] doubles = {Math.PI, 2.71, 1.41, 1.73};
        System.out.println(Arrays.toString(doubles));

        String[] strings = {"Hello", "Array", "to", "String", "Example"};
        System.out.println(Arrays.toString(strings));
    }
}
