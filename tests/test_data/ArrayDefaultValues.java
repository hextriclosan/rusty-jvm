package samples.arrays.defaultvalues;

public class ArrayDefaultValues {

    public static void main(String[] args) {
        // Integer types
        int[] intArray = new int[1];
        long[] longArray = new long[1];
        byte[] byteArray = new byte[1];
        short[] shortArray = new short[1];

        // Floating-point types
        float[] floatArray = new float[1];
        double[] doubleArray = new double[1];

        // Character type
        char[] charArray = new char[1];

        // Boolean type
        boolean[] booleanArray = new boolean[1];

        // Object type
        String[] stringArray = new String[1];

        // 2D array
        int[][] int2DArray = new int[1][1];
        float[][] float2DArray = new float[1][1];

        // Print default values
        System.out.println("Default value in int array: " + intArray[0]);
        System.out.println("Default value in long array: " + longArray[0]);
        System.out.println("Default value in byte array: " + byteArray[0]);
        System.out.println("Default value in short array: " + shortArray[0]);
        System.out.println("Default value in float array: " + floatArray[0]);
        System.out.println("Default value in double array: " + doubleArray[0]);
        System.out.println("Default value in char array: " + (int)charArray[0]);
        System.out.println("Default value in boolean array: " + booleanArray[0]);
        System.out.println("Default value in String array: " + stringArray[0]);
        System.out.println("Default value in 2D int array: " + int2DArray[0][0]);
        System.out.println("Default value in 2D float array: " + float2DArray[0][0]);
    }
}
