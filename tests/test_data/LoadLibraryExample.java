package samples.javacore.loadlibrary.example;

class LoadLibraryExample {

    // Primitive operations
    private static native int sum(int a, int b);
    private static native long sum(long a, long b);
    private static native double multiply(double a, double b);

    // Instance method
    private native int sumInstance(int a, int b);

    // Boolean test
    private static native boolean isPositive(int value);

    // Array example
    private static native int arraySum(int[] values);

    // Object reference example
    private static native String hello(String name);

    // Void return
    private static native void printMessage(String message);
    private static native void printFloat(float value);

    static {
        System.loadLibrary("jni_test_lib");
    }

    public static void main(String[] args) {
        int intFirst = 40;
        int intSecond = 2;
        int intResult = sum(intFirst, intSecond);
        System.out.printf("int sum(%d, %d) = %d%n", intFirst, intSecond, intResult);

        long longFirst = -99_999_999_999_999L;
        long longSecond = 100_000_000_000_000L;
        long longResult = sum(longFirst, longSecond);
        System.out.printf("long sum(%d, %d) = %d%n", longFirst, longSecond, longResult);

        double doubleFirst = 3.5;
        double doubleSecond = 2.0;
        double doubleResult = multiply(doubleFirst, doubleSecond);
        System.out.printf("double multiply(%f, %f) = %f%n", doubleFirst, doubleSecond, doubleResult);

        int checkValue = -5;
        boolean positive = isPositive(checkValue);
        System.out.printf("isPositive(%d) = %b%n", checkValue, positive);

//         int[] values = {1, 2, 3, 4};
//         int arrSum = arraySum(values);
//         System.out.println("arraySum = " + arrSum);

//         String greeting = hello("JNI");
//         System.out.println(greeting);

//         printMessage("Message from Java");

        printFloat(3.1415f);

        LoadLibraryExample obj = new LoadLibraryExample();
        int instanceResult = obj.sumInstance(intFirst, intSecond);
        System.out.printf("int sumInstance(%d, %d) = %d%n", intFirst, intSecond, instanceResult);
    }
}
