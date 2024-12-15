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
        System.out.print("Default value in int array: ");
        System.out.println(intArray[0]);
        System.out.print("Default value in long array: ");
        System.out.println(longArray[0]);
        System.out.print("Default value in byte array: ");
        System.out.println(byteArray[0]);
        System.out.print("Default value in short array: ");
        System.out.println(shortArray[0]);
        System.out.print("Default value in float array: ");
        System.out.println(floatArray[0]);
        System.out.print("Default value in double array: ");
        System.out.println(doubleArray[0]);
        System.out.print("Default value in char array: ");
        System.out.println((int)charArray[0]);
        System.out.print("Default value in boolean array: ");
        System.out.println(booleanArray[0]);
        System.out.print("Default value in String array: ");
        System.out.println(stringArray[0]);
        System.out.print("Default value in 2D int array: ");
        System.out.println(int2DArray[0][0]);
        System.out.print("Default value in 2D float array: ");
        System.out.println(float2DArray[0][0]);
    }
}
