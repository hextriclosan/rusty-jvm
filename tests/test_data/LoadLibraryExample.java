package samples.javacore.loadlibrary.example;

import java.util.Arrays;
import java.util.List;

class LoadLibraryExample {

    // Primitive operations
    private static native byte sum(byte a, byte b);
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

    private static native int getJniVersion();

    static {
        System.loadLibrary("jni_test_lib");
    }

    public static void main(String[] args) {
        byte byteFirst = -128;
        byte byteSecond = 28;
        byte byteResult = sum(byteFirst, byteSecond);
        System.out.printf("byte sum(%d, %d) = %d%n", byteFirst, byteSecond, byteResult);

        int intFirst = 40;
        int intSecond = 2;
        int intResult = sum(intFirst, intSecond);
        System.out.printf("int sum(%d, %d) = %d%n", intFirst, intSecond, intResult);

        long longFirst = -99_999_999_999_9999L;
        long longSecond = 1000_000_000L;
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

        int jniVersion = getJniVersion();
        System.out.printf("JNI version: 0x%08x%n", jniVersion);

        StringOperationsDemo.runDemo();
        ArrayOperationsDemo.runDemo();
    }
}

class StringOperationsDemo {
    private static native int getStringLength(Object input);

    public static void runDemo() {
        GetStringLengthDemo();
    }

    private static void GetStringLengthDemo() {
        System.out.println();
        System.out.println("=== GetStringLength ===");
        String testString = "Hello, JNI 💅☕️!";
        int length = getStringLength(testString);
        System.out.printf("Length of '%s' is %d%n", testString, length);

        Object notAString = List.of(1, 2, 3);
        int notAStringLength = getStringLength(notAString);
        System.out.printf("Length of '%s' is %d%n", notAString, notAStringLength);
    }
}

class ArrayOperationsDemo {
    private static native int GetArrayLength(Object input);
    private static native Object NewObjectArray(int length, Object elementClass, Object initialElement);
    private static native Object GetObjectArrayElement(Object array, int index);
    private static native void SetObjectArrayElement(Object array, int index, Object value);

    public static void runDemo() {
        GetArrayLengthDemo();
        NewObjectArrayDemo();
        GetObjectArrayElementDemo();
        SetObjectArrayElementDemo();
    }

    private static void GetArrayLengthDemo() {
        System.out.println();
        System.out.println("=== GetArrayLength ===");

        byte[] byteArr = {1, 2, 3, 4, 5};
        int byteArrLength = GetArrayLength(byteArr);
        System.out.printf("Length of array %s is %d%n", Arrays.toString(byteArr), byteArrLength);

        String[] stringArr = {"one", "two", "three"};
        int stringArrLength = GetArrayLength(stringArr);
        System.out.printf("Length of array %s is %d%n", Arrays.toString(stringArr), stringArrLength);

        String notAnArray = "Not an array";
        int notAnArrayLength = GetArrayLength(notAnArray);
        System.out.printf("Length of '%s' is %d%n", notAnArray, notAnArrayLength);
    }

    private static void NewObjectArrayDemo() {
        System.out.println();
        System.out.println("=== NewObjectArray ===");

        Object stringArray = NewObjectArray(5, String.class, "Hi");
        System.out.printf("Created String array: %s%n", Arrays.toString((String[])stringArray));

        Object nullStringArray = NewObjectArray(5, String.class, null);
        System.out.printf("Created null String array: %s%n", Arrays.toString((String[])nullStringArray));

        Object string2DArray = NewObjectArray(5, String[].class, new String[] { "Hi" });
        System.out.printf("Created String 2D array: %s%n", Arrays.deepToString((String[][])string2DArray));

        Object int2DArray = NewObjectArray(3, int[].class, new int[] { 10, 20, 30 });
        System.out.printf("Created int 2D array: %s%n", Arrays.deepToString((int[][])int2DArray));
    }

    private static void GetObjectArrayElementDemo() {
        System.out.println();
        System.out.println("=== GetObjectArrayElement ===");

        String[] stringArr = {"one", "two", "three"};
        Object element1 = GetObjectArrayElement(stringArr, 1);
        System.out.printf("Element at index 1 of %s is '%s'%n", Arrays.toString(stringArr), element1);

        String[][] string2DArray = {{"a", "b"}, {"c", "d"}};
        Object element2 = GetObjectArrayElement(string2DArray, 0);
        System.out.printf("Element at index 0 of %s is '%s'%n", Arrays.deepToString(string2DArray), Arrays.toString((String[])element2));
    }

    private static void SetObjectArrayElementDemo() {
        System.out.println();
        System.out.println("=== SetObjectArrayElement ===");

        String[] stringArr = {"one", "two", "three"};
        SetObjectArrayElement(stringArr, 1, "twenty");
        System.out.printf("Array after modification '%s'%n", Arrays.toString(stringArr));

        SetObjectArrayElement(stringArr, 0, null);
        System.out.printf("Array after setting null '%s'%n", Arrays.toString(stringArr));

        String[][] string2DArray = {{"a", "b"}, {"c", "d"}};
        SetObjectArrayElement(string2DArray, 0, new String[]{ "x", "y" });
        System.out.printf("2D Array after modification '%s'%n", Arrays.deepToString(string2DArray));
    }
}
