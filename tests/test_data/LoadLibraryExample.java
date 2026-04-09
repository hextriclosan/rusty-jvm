package samples.javacore.loadlibrary.example;

import java.util.Arrays;
import java.util.List;
import java.util.stream.IntStream;

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

        int[] values = {1, 2, 3, 4};
        int arrSum = arraySum(values);
        System.out.println("arraySum = " + arrSum);

        String greeting = hello("Hi éA\u0000𝄞💅☕️");
        System.out.println(greeting);

        printMessage("Hello from Java");

        printFloat(3.1415f);

        LoadLibraryExample obj = new LoadLibraryExample();
        int instanceResult = obj.sumInstance(intFirst, intSecond);
        System.out.printf("int sumInstance(%d, %d) = %d%n", intFirst, intSecond, instanceResult);

        int jniVersion = getJniVersion();
        System.out.printf("JNI version: 0x%08x%n", jniVersion);

        StringOperationsDemo.runDemo();
        ArrayOperationsDemo.runDemo();
        StaticFieldDemo.runDemo();
        ObjectOperationsDemo.runDemo();
        ObjectFieldDemo objectFieldDemo = new ObjectFieldDemo();
        objectFieldDemo.runDemo();
        StaticMethodsDemo.runDemo();

        InstanceMethodsDemo instanceMethodDemo = new InstanceMethodsDemo();
        instanceMethodDemo.runDemo();

        VirtualDispatchDemo virtualDispatchDemo = new VirtualDispatchDemo();
        virtualDispatchDemo.runDemo();
    }
}

class StringOperationsDemo {
    private static native int GetStringLength(Object input);
    private static native String NewString(char[] input);
    private static native char[] GetStringChars(String input);
    private static native String NewStringUTF();
    private static native int GetStringUTFLength(String input);
    private static native long GetStringUTFLengthAsLong(String input);
    private static native String GetStringUTFChars(String input);

    public static void runDemo() {
        GetStringLengthDemo();
        NewStringDemo();
        GetStringCharsDemo();
        NewStringUTFDemo();
        GetStringUTFLengthAsLongDemo();
        GetStringUTFCharsDemo();
    }

    private static void GetStringLengthDemo() {
        System.out.println();
        System.out.println("=== GetStringLength ===");
        String testString = "Hello, JNI 💅☕️!";
        int length = GetStringLength(testString);
        System.out.printf("Length of '%s' is %d%n", testString, length);

        Object notAString = List.of(1, 2, 3);
        int notAStringLength = GetStringLength(notAString);
        System.out.printf("Length of '%s' is %d%n", notAString, notAStringLength);
    }

    private static void NewStringDemo() {
        System.out.println();
        System.out.println("=== NewString ===");
        char[] input = {'H', 'e', 'l', 'l', 'o', ',', ' ', 'J', 'N', 'I', ' ', '\uD83D', '\uDC85', '!'};

        String result = NewString(input);
        System.out.printf("Result of newString with input %s: %s%n", Arrays.toString(input), result);
    }

    private static void GetStringCharsDemo() {
        System.out.println();
        System.out.println("=== GetStringChars ===");

        String utf16Input = "Hello, JNI 💅☕️!";
        char[] utf16Chars = GetStringChars(utf16Input);
        System.out.printf("Result of GetStringChars with input '%s': %s%n", utf16Input, Arrays.toString(toUtf16CodeUnits(utf16Chars)));

        String latinInput = "abc";
        char[] latinChars = GetStringChars(latinInput);
        System.out.printf("Result of GetStringChars with input '%s': %s%n", latinInput, Arrays.toString(toUtf16CodeUnits(latinChars)));
    }

    private static int[] toUtf16CodeUnits(char[] chars) {
        return IntStream.range(0, chars.length)
            .map(i -> chars[i])
            .toArray();
    }

    private static void NewStringUTFDemo() {
        System.out.println();
        System.out.println("=== NewStringUTF ===");
        String result = NewStringUTF();
        System.out.printf("Result of newStringUTF: %s%n", result);
    }

    private static void GetStringUTFLengthAsLongDemo() {
        System.out.println();
        System.out.println("=== GetStringUTFLengthAsLong ===");
        String testString = "éA\u0000𝄞";
        long utfLengthLong = GetStringUTFLengthAsLong(testString);
        int utfLengthInt = GetStringUTFLength(testString);
        int charsLength = GetStringLength(testString);
        System.out.printf("Length of '%s': utfLengthLong=%d, utfLengthInt=%d charsLength=%d%n", testString, utfLengthLong, utfLengthInt, charsLength);

        testString = "éabc";
        utfLengthLong = GetStringUTFLengthAsLong(testString);
        utfLengthInt = GetStringUTFLength(testString);
        charsLength = GetStringLength(testString);
        System.out.printf("Length of '%s': utfLengthLong=%d, utfLengthInt=%d charsLength=%d%n", testString, utfLengthLong, utfLengthInt, charsLength);
    }

    private static void GetStringUTFCharsDemo() {
        System.out.println();
        System.out.println("=== GetStringUTFChars ===");

        String utf16Input = "A\u0000𝄞";
        String utf16Output = GetStringUTFChars(utf16Input);
        System.out.printf("Result of GetStringUTFChars with input '%s': '%s'%n", utf16Input, utf16Output);

        String latinInput = "abc";
        String latinOutput = GetStringUTFChars(latinInput);
        System.out.printf("Result of GetStringUTFChars with input '%s': '%s'%n", latinInput, latinOutput);
    }
}

class ArrayOperationsDemo {
    private static native int GetArrayLength(Object input);
    private static native Object NewObjectArray(int length, Object elementClass, Object initialElement);
    private static native Object GetObjectArrayElement(Object array, int index);
    private static native void SetObjectArrayElement(Object array, int index, Object value);

    private static native boolean[] NewBooleanArray(int length);
    private static native byte[] NewByteArray(int length);
    private static native char[] NewCharArray(int length);
    private static native short[] NewShortArray(int length);
    private static native int[] NewIntArray(int length);
    private static native long[] NewLongArray(int length);
    private static native float[] NewFloatArray(int length);
    private static native double[] NewDoubleArray(int length);

    private static native void GetAndReleaseBooleanArrayDemo(boolean[] array);
    private static native void GetAndReleaseByteArrayDemo(byte[] array);
    private static native void GetAndReleaseCharArrayDemo(char[] array);
    private static native void GetAndReleaseShortArrayDemo(short[] array);
    private static native void GetAndReleaseIntArrayDemo(int[] array);
    private static native void GetAndReleaseLongArrayDemo(long[] array);
    private static native void GetAndReleaseFloatArrayDemo(float[] array);
    private static native void GetAndReleaseDoubleArrayDemo(double[] array);

    private static native void SetBooleanArrayRegionDemo(boolean[] dest, int start, int length, boolean[] source);
    private static native void SetByteArrayRegionDemo(byte[] dest, int start, int length, byte[] source);
    private static native void SetCharArrayRegionDemo(char[] dest, int start, int length, char[] source);
    private static native void SetShortArrayRegionDemo(short[] dest, int start, int length, short[] source);
    private static native void SetIntArrayRegionDemo(int[] dest, int start, int length, int[] source);
    private static native void SetLongArrayRegionDemo(long[] dest, int start, int length, long[] source);
    private static native void SetFloatArrayRegionDemo(float[] dest, int start, int length, float[] source);
    private static native void SetDoubleArrayRegionDemo(double[] dest, int start, int length, double[] source);

    private static native boolean[] GetBooleanArrayRegionDemo(boolean[] from, int start, int length);
    private static native byte[] GetByteArrayRegionDemo(byte[] from, int start, int length);
    private static native char[] GetCharArrayRegionDemo(char[] from, int start, int length);
    private static native short[] GetShortArrayRegionDemo(short[] from, int start, int length);
    private static native int[] GetIntArrayRegionDemo(int[] from, int start, int length);
    private static native long[] GetLongArrayRegionDemo(long[] from, int start, int length);
    private static native float[] GetFloatArrayRegionDemo(float[] from, int start, int length);
    private static native double[] GetDoubleArrayRegionDemo(double[] from, int start, int length);

    public static void runDemo() {
        GetArrayLengthDemo();
        NewObjectArrayDemo();
        GetObjectArrayElementDemo();
        SetObjectArrayElementDemo();
        NewPrimitiveArrayDemo();
        GetAndReleasePrimitiveArrayElementsDemo();
        GetAndSetPrimitiveArrayRegionDemo();
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

    private static void NewPrimitiveArrayDemo() {
        System.out.println();
        System.out.println("=== New<PrimitiveType>Array ===");

        boolean[] boolArr = NewBooleanArray(3);
        System.out.printf("Created boolean array: %s%n", Arrays.toString(boolArr));

        byte[] byteArr = NewByteArray(3);
        System.out.printf("Created byte array: %s%n", Arrays.toString(byteArr));

        char[] charArr = NewCharArray(3);
        System.out.printf("Created char array: %s%n", Arrays.toString(charArr));

        short[] shortArr = NewShortArray(3);
        System.out.printf("Created short array: %s%n", Arrays.toString(shortArr));

        int[] intArr = NewIntArray(3);
        System.out.printf("Created int array: %s%n", Arrays.toString(intArr));

        long[] longArr = NewLongArray(3);
        System.out.printf("Created long array: %s%n", Arrays.toString(longArr));

        float[] floatArr = NewFloatArray(3);
        System.out.printf("Created float array: %s%n", Arrays.toString(floatArr));

        double[] doubleArr = NewDoubleArray(3);
        System.out.printf("Created double array: %s%n", Arrays.toString(doubleArr));
    }

    private static void GetAndReleasePrimitiveArrayElementsDemo() {
        System.out.println();
        System.out.println("=== Get<PrimitiveType>ArrayElements and Release<PrimitiveType>ArrayElements ===");

        boolean[] boolArr = {true, false, false};
        System.out.printf("Boolean array before modification: %s%n", Arrays.toString(boolArr));
        GetAndReleaseBooleanArrayDemo(boolArr);
        System.out.printf("Boolean array after modification: %s%n", Arrays.toString(boolArr));

        byte[] byteArr = {-128, 1, 127};
        System.out.printf("Byte array before modification: %s%n", Arrays.toString(byteArr));
        GetAndReleaseByteArrayDemo(byteArr);
        System.out.printf("Byte array after modification: %s%n", Arrays.toString(byteArr));

        char[] charArr = {'Ї', 'A', '\u2620'};
        System.out.printf("Char array before modification: %s%n", Arrays.toString(charArr));
        GetAndReleaseCharArrayDemo(charArr);
        System.out.printf("Char array after modification: %s%n", Arrays.toString(charArr));

        short[] shortArr = {-32768, 1337, 32767};
        System.out.printf("Short array before modification: %s%n", Arrays.toString(shortArr));
        GetAndReleaseShortArrayDemo(shortArr);
        System.out.printf("Short array after modification: %s%n", Arrays.toString(shortArr));

        int[] intArr = {-2_000_000_000, 1337, 2_000_000_000};
        System.out.printf("Int array before modification: %s%n", Arrays.toString(intArr));
        GetAndReleaseIntArrayDemo(intArr);
        System.out.printf("Int array after modification: %s%n", Arrays.toString(intArr));

        long[] longArr = {-9_000_000_000_000_000_000L, 1337L, 9_000_000_000_000_000_000L};
        System.out.printf("Long array before modification: %s%n", Arrays.toString(longArr));
        GetAndReleaseLongArrayDemo(longArr);
        System.out.printf("Long array after modification: %s%n", Arrays.toString(longArr));

        float[] floatArr = {3.14f, 2.71f, 1.41f};
        System.out.printf("Float array before modification: %s%n", Arrays.toString(floatArr));
        GetAndReleaseFloatArrayDemo(floatArr);
        System.out.printf("Float array after modification: %s%n", Arrays.toString(floatArr));

        double[] doubleArr = {Math.PI, 2.71, 1.41};
        System.out.printf("Double array before modification: %s%n", Arrays.toString(doubleArr));
        GetAndReleaseDoubleArrayDemo(doubleArr);
        System.out.printf("Double array after modification: %s%n", Arrays.toString(doubleArr));
    }

    private static void GetAndSetPrimitiveArrayRegionDemo() {
        System.out.println();
        System.out.println("=== Get<PrimitiveType>ArrayRegion and Set<PrimitiveType>ArrayRegion ===");

        boolean[] boolArr = new boolean[6];
        SetBooleanArrayRegionDemo(boolArr, 2, 3, new boolean[] { true, true, true });
        System.out.printf("Boolean array after setting: %s%n", Arrays.toString(boolArr));
        boolean[] gotBoolArr = GetBooleanArrayRegionDemo(boolArr, 1, 4);
        System.out.printf("Got boolean array region: %s%n", Arrays.toString(gotBoolArr));

        byte[] byteArr = new byte[6];
        SetByteArrayRegionDemo(byteArr, 2, 3, new byte[] { -128, 1, 127 });
        System.out.printf("Byte array after setting: %s%n", Arrays.toString(byteArr));
        byte[] gotByteArr = GetByteArrayRegionDemo(byteArr, 1, 4);
        System.out.printf("Got byte array region: %s%n", Arrays.toString(gotByteArr));

        char[] charArr = new char[] {'A', 'B', 'C', 'D', 'E', 'F'};
        SetCharArrayRegionDemo(charArr, 2, 3, new char[] { 'Ї', 'X', '\u2620'});
        System.out.printf("Char array after setting: %s%n", Arrays.toString(charArr));
        char[] gotCharArr = GetCharArrayRegionDemo(charArr, 1, 4);
        System.out.printf("Got char array region: %s%n", Arrays.toString(gotCharArr));

        short[] shortArr = new short[6];
        SetShortArrayRegionDemo(shortArr, 2, 3, new short[] { -32768, 1337, 32767 });
        System.out.printf("Short array after setting: %s%n", Arrays.toString(shortArr));
        short[] gotShortArr = GetShortArrayRegionDemo(shortArr, 1, 4);
        System.out.printf("Got short array region: %s%n", Arrays.toString(gotShortArr));

        int[] intArr = new int[6];
        SetIntArrayRegionDemo(intArr, 2, 3, new int[] { -2_000_000_000, 1337, 2_000_000_000 });
        System.out.printf("Int array after setting: %s%n", Arrays.toString(intArr));
        int[] gotIntArr = GetIntArrayRegionDemo(intArr, 1, 4);
        System.out.printf("Got int array region: %s%n", Arrays.toString(gotIntArr));

        long[] longArr = new long[6];
        SetLongArrayRegionDemo(longArr, 2, 3, new long[] { -9_000_000_000_000_000_000L, 1337L, 9_000_000_000_000_000_000L });
        System.out.printf("Long array after setting: %s%n", Arrays.toString(longArr));
        long[] gotLongArr = GetLongArrayRegionDemo(longArr, 1, 4);
        System.out.printf("Got long array region: %s%n", Arrays.toString(gotLongArr));

        float[] floatArr = new float[6];
        SetFloatArrayRegionDemo(floatArr, 2, 3, new float[] { 3.14f, 2.71f, 1.41f });
        System.out.printf("Float array after setting: %s%n", Arrays.toString(floatArr));
        float[] gotFloatArr = GetFloatArrayRegionDemo(floatArr, 1, 4);
        System.out.printf("Got float array region: %s%n", Arrays.toString(gotFloatArr));

        double[] doubleArr = new double[6];
        SetDoubleArrayRegionDemo(doubleArr, 2, 3, new double[] { Math.PI, 2.71, 1.41 });
        System.out.printf("Double array after setting: %s%n", Arrays.toString(doubleArr));
        double[] gotDoubleArr = GetDoubleArrayRegionDemo(doubleArr, 1, 4);
        System.out.printf("Got double array region: %s%n", Arrays.toString(gotDoubleArr));
    }
}

class StaticFieldDemo {
    private static String staticObjectField = "Initial static string";
    public static boolean staticBooleanField = false;
    private static byte staticByteField = 127;
    private static char staticCharField = 'A';
    private static short staticShortField = 1337;
    private static int staticIntField = 42;
    private static long staticLongField = 9_000_000_000_000_000_000L;
    private static float staticFloatField = 3.14f;
    private static double staticDoubleField = Math.PI;

    private static native void processStaticObjectField(String fieldName, String sig, Object newValue);
    private static native void processStaticBooleanField(String fieldName, String sig, boolean newValue);
    private static native void processStaticByteField(String fieldName, String sig, byte newValue);
    private static native void processStaticCharField(String fieldName, String sig, char newValue);
    private static native void processStaticShortField(String fieldName, String sig, short newValue);
    private static native void processStaticIntField(String fieldName, String sig, int newValue);
    private static native void processStaticLongField(String fieldName, String sig, long newValue);
    private static native void processStaticFloatField(String fieldName, String sig, float newValue);
    private static native void processStaticDoubleField(String fieldName, String sig, double newValue);

    public static void runDemo() {
        System.out.println();
        System.out.println("=== Static Field Demo ===");

        System.out.print("staticObjectField: ");
        processStaticObjectField("staticObjectField", "Ljava/lang/String;", "I'm a brand new");
        System.out.printf("staticObjectField: new value=%s%n", staticObjectField);

        System.out.print("staticBooleanField: ");
        processStaticBooleanField("staticBooleanField", "Z", true);
        System.out.printf("staticBooleanField: new value=%b%n", staticBooleanField);

        System.out.print("staticByteField: ");
        processStaticByteField("staticByteField", "B", (byte)-128);
        System.out.printf("staticByteField: new value=%d%n", staticByteField);

        System.out.print("staticCharField: ");
        processStaticCharField("staticCharField", "C", 'Ї');
        System.out.printf("staticCharField: new value=%c%n", staticCharField);

        System.out.print("staticShortField: ");
        processStaticShortField("staticShortField", "S", (short) -32768);
        System.out.printf("staticShortField: new value=%d%n", staticShortField);

        System.out.print("staticIntField: ");
        processStaticIntField("staticIntField", "I", -2_000_000_000);
        System.out.printf("staticIntField: new value=%d%n", staticIntField);

        System.out.print("staticLongField: ");
        processStaticLongField("staticLongField", "J", -9_000_000_000_000_000_000L);
        System.out.printf("staticLongField: new value=%d%n", staticLongField);

        System.out.print("staticFloatField: ");
        processStaticFloatField("staticFloatField", "F", 2.71f);
        System.out.printf("staticFloatField: new value=%f%n", staticFloatField);

        System.out.print("staticDoubleField: ");
        processStaticDoubleField("staticDoubleField", "D", Math.E);
        System.out.printf("staticDoubleField: new value=%.16f%n", staticDoubleField);
    }
}

class ObjectOperationsDemo {
    private static native Class<?> GetObjectClass(Object obj);

    public static void runDemo() {
        System.out.println();
        System.out.println("=== Object Operations Demo ===");

        GetObjectClassDemo();
    }

    private static void GetObjectClassDemo() {
        printGetObjectClass("Hello, JNI 💅☕️!");
        printGetObjectClass(42);
        printGetObjectClass(List.of("one", "two", "three"));
        printGetObjectClass(new int[] {1, 2, 3});
    }

    private static void printGetObjectClass(Object obj) {
        Class<?> clazz = GetObjectClass(obj);
        if (obj instanceof int[] ints) {
            System.out.printf("Class of array object '%s' is %s%n", Arrays.toString(ints), clazz.getName());
        } else {
            System.out.printf("Class of object '%s' is %s%n", obj, clazz.getName());
        }
    }
}

class ObjectFieldDemo {
    private String objectStringField = "Initial object string";
    public boolean objectBooleanField = false;
    private byte objectByteField = 127;
    private char objectCharField = 'A';
    private short objectShortField = 1337;
    private int objectIntField = 42;
    private long objectLongField = 9_000_000_000_000_000_000L;
    private float objectFloatField = 3.14f;
    private double objectDoubleField = Math.PI;

    private native void processObjectStringField(String fieldName, String sig, Object newValue);
    private native void processObjectBooleanField(String fieldName, String sig, boolean newValue);
    private native void processObjectByteField(String fieldName, String sig, byte newValue);
    private native void processObjectCharField(String fieldName, String sig, char newValue);
    private native void processObjectShortField(String fieldName, String sig, short newValue);
    private native void processObjectIntField(String fieldName, String sig, int newValue);
    private native void processObjectLongField(String fieldName, String sig, long newValue);
    private native void processObjectFloatField(String fieldName, String sig, float newValue);
    private native void processObjectDoubleField(String fieldName, String sig, double newValue);

    public void runDemo() {
        System.out.println();
        System.out.println("=== Object Field Demo ===");

        System.out.print("objectStringField: ");
        processObjectStringField("objectStringField", "Ljava/lang/String;", "I'm a brand new");
        System.out.printf("objectStringField: new value=%s%n", objectStringField);

        System.out.print("objectBooleanField: ");
        processObjectBooleanField("objectBooleanField", "Z", true);
        System.out.printf("objectBooleanField: new value=%b%n", objectBooleanField);

        System.out.print("objectByteField: ");
        processObjectByteField("objectByteField", "B", (byte)-128);
        System.out.printf("objectByteField: new value=%d%n", objectByteField);

        System.out.print("objectCharField: ");
        processObjectCharField("objectCharField", "C", 'Ї');
        System.out.printf("objectCharField: new value=%c%n", objectCharField);

        System.out.print("objectShortField: ");
        processObjectShortField("objectShortField", "S", (short) -32768);
        System.out.printf("objectShortField: new value=%d%n", objectShortField);

        System.out.print("objectIntField: ");
        processObjectIntField("objectIntField", "I", -2_000_000_000);
        System.out.printf("objectIntField: new value=%d%n", objectIntField);

        System.out.print("objectLongField: ");
        processObjectLongField("objectLongField", "J", -9_000_000_000_000_000_000L);
        System.out.printf("objectLongField: new value=%d%n", objectLongField);

        System.out.print("objectFloatField: ");
        processObjectFloatField("objectFloatField", "F", 2.71f);
        System.out.printf("objectFloatField: new value=%f%n", objectFloatField);

        System.out.print("objectDoubleField: ");
        processObjectDoubleField("objectDoubleField", "D", Math.E);
        System.out.printf("objectDoubleField: new value=%.16f%n", objectDoubleField);
    }
}

class StaticMethodsDemo {
    private static native Object StaticObjectMethodDemo(String methodName, String signature, boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l);
    private static native boolean StaticBooleanMethodDemo(String methodName, String signature, boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l);
    private static native byte StaticByteMethodDemo(String methodName, String signature, boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l);
    private static native char StaticCharMethodDemo(String methodName, String signature, boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l);
    private static native short StaticShortMethodDemo(String methodName, String signature, boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l);
    private static native int StaticIntMethodDemo(String methodName, String signature, boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l);
    private static native long StaticLongMethodDemo(String methodName, String signature, boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l);
    private static native float StaticFloatMethodDemo(String methodName, String signature, boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l);
    private static native double StaticDoubleMethodDemo(String methodName, String signature, boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l);
    private static native void StaticVoidMethodDemo(String methodName, String signature, boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l);

    private static final String SIG_TMPL = "(ZBCSIJFDLjava/lang/Object;)%s";
    private static final String MSG_TMPL = "%s called with %b, %d, %c, %d, %d, %d, %.12f, %.12f, %s%n";
    private static final boolean Z = true;
    private static final byte B = -128;
    private static final char C = 'Ї';
    private static final short S = -32768;
    private static final int I = -2_000_000_000;
    private static final long J = -9_000_000_000_000_000_000L;
    private static final float F = 3.14f;
    private static final double D = Math.PI;
    private static final Object L = "Hi";

    public static void runDemo() {
        System.out.println();
        System.out.println("=== Static Methods Demo ===");

        Object objectResult = StaticObjectMethodDemo("objectMethodToCall", SIG_TMPL.formatted("Ljava/lang/Object;"), Z, B, C, S, I, J, F, D, L);
        System.out.printf("StaticObjectMethodDemo -> %s%n", objectResult);

        boolean boolResult = StaticBooleanMethodDemo("booleanMethodToCall", SIG_TMPL.formatted("Z"), Z, B, C, S, I, J, F, D, L);
        System.out.printf("StaticBooleanMethodDemo -> %b%n", boolResult);

        byte byteResult = StaticByteMethodDemo("byteMethodToCall", SIG_TMPL.formatted("B"), Z, B, C, S, I, J, F, D, L);
        System.out.printf("StaticByteMethodDemo -> %d%n", byteResult);

        char charResult = StaticCharMethodDemo("charMethodToCall", SIG_TMPL.formatted("C"), Z, B, C, S, I, J, F, D, L);
        System.out.printf("StaticCharMethodDemo -> %c%n", charResult);

        short shortResult = StaticShortMethodDemo("shortMethodToCall", SIG_TMPL.formatted("S"), Z, B, C, S, I, J, F, D, L);
        System.out.printf("StaticShortMethodDemo -> %d%n", shortResult);

        int intResult = StaticIntMethodDemo("intMethodToCall", SIG_TMPL.formatted("I"), Z, B, C, S, I, J, F, D, L);
        System.out.printf("StaticIntMethodDemo -> %d%n", intResult);

        long longResult = StaticLongMethodDemo("longMethodToCall", SIG_TMPL.formatted("J"), Z, B, C, S, I, J, F, D, L);
        System.out.printf("StaticLongMethodDemo -> %d%n", longResult);

        float floatResult = StaticFloatMethodDemo("floatMethodToCall", SIG_TMPL.formatted("F"), Z, B, C, S, I, J, F, D, L);
        System.out.printf("StaticFloatMethodDemo -> %f%n", floatResult);

        double doubleResult = StaticDoubleMethodDemo("doubleMethodToCall", SIG_TMPL.formatted("D"), Z, B, C, S, I, J, F, D, L);
        System.out.printf("StaticDoubleMethodDemo -> %f%n", doubleResult);

        StaticVoidMethodDemo("voidMethodToCall", SIG_TMPL.formatted("V"), Z, B, C, S, I, J, F, D, L);
    }

    private static Object objectMethodToCall(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l) {
        System.out.printf(MSG_TMPL, "objectMethodToCall", z, b, c, s, i, j, f, d, l);
        return "I'm a result from Java!";
    }

    private static boolean booleanMethodToCall(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l) {
        System.out.printf(MSG_TMPL, "booleanMethodToCall", z, b, c, s, i, j, f, d, l);
        return true;
    }

    private static byte byteMethodToCall(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l) {
        System.out.printf(MSG_TMPL, "byteMethodToCall", z, b, c, s, i, j, f, d, l);
        return -128;
    }

    private static char charMethodToCall(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l) {
        System.out.printf(MSG_TMPL, "charMethodToCall", z, b, c, s, i, j, f, d, l);
        return 'Ї';
    }

    private static short shortMethodToCall(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l) {
        System.out.printf(MSG_TMPL, "shortMethodToCall", z, b, c, s, i, j, f, d, l);
        return -32768;
    }

    private static int intMethodToCall(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l) {
        System.out.printf(MSG_TMPL, "intMethodToCall", z, b, c, s, i, j, f, d, l);
        return -2_000_000_000;
    }

    private static long longMethodToCall(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l) {
        System.out.printf(MSG_TMPL, "longMethodToCall", z, b, c, s, i, j, f, d, l);
        return -9_000_000_000_000_000_000L;
    }

    private static float floatMethodToCall(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l) {
        System.out.printf(MSG_TMPL, "floatMethodToCall", z, b, c, s, i, j, f, d, l);
        return 2.71f;
    }

    private static double doubleMethodToCall(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l) {
        System.out.printf(MSG_TMPL, "doubleMethodToCall", z, b, c, s, i, j, f, d, l);
        return Math.E;
    }

    private static void voidMethodToCall(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l) {
        System.out.printf(MSG_TMPL, "voidMethodToCall", z, b, c, s, i, j, f, d, l);
    }
}

class InstanceMethodsDemo {
    private native Object InstanceObjectMethodDemo(String methodName, String signature, boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l);
    private native boolean InstanceBooleanMethodDemo(String methodName, String signature, boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l);
    private native byte InstanceByteMethodDemo(String methodName, String signature, boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l);
    private native char InstanceCharMethodDemo(String methodName, String signature, boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l);
    private native short InstanceShortMethodDemo(String methodName, String signature, boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l);
    private native int InstanceIntMethodDemo(String methodName, String signature, boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l);
    private native long InstanceLongMethodDemo(String methodName, String signature, boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l);
    private native float InstanceFloatMethodDemo(String methodName, String signature, boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l);
    private native double InstanceDoubleMethodDemo(String methodName, String signature, boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l);
    private native void InstanceVoidMethodDemo(String methodName, String signature, boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l);

    private static final String SIG_TMPL = "(ZBCSIJFDLjava/lang/Object;)%s";
    private static final String MSG_TMPL = "%s called with %b, %d, %c, %d, %d, %d, %.12f, %.12f, %s%n";
    private static final boolean Z = true;
    private static final byte B = -128;
    private static final char C = 'Ї';
    private static final short S = -32768;
    private static final int I = -2_000_000_000;
    private static final long J = -9_000_000_000_000_000_000L;
    private static final float F = 3.14f;
    private static final double D = Math.PI;
    private static final Object L = "Hi";

    public void runDemo() {
        System.out.println();
        System.out.println("=== Instance Methods Demo ===");

        Object objectResult = InstanceObjectMethodDemo("instanceObjectMethodToCall", SIG_TMPL.formatted("Ljava/lang/Object;"), Z, B, C, S, I, J, F, D, L);
        System.out.printf("InstanceObjectMethodDemo -> %s%n", objectResult);

        boolean boolResult = InstanceBooleanMethodDemo("instanceBooleanMethodToCall", SIG_TMPL.formatted("Z"), Z, B, C, S, I, J, F, D, L);
        System.out.printf("InstanceBooleanMethodDemo -> %b%n", boolResult);

        byte byteResult = InstanceByteMethodDemo("instanceByteMethodToCall", SIG_TMPL.formatted("B"), Z, B, C, S, I, J, F, D, L);
        System.out.printf("InstanceByteMethodDemo -> %d%n", byteResult);

        char charResult = InstanceCharMethodDemo("instanceCharMethodToCall", SIG_TMPL.formatted("C"), Z, B, C, S, I, J, F, D, L);
        System.out.printf("InstanceCharMethodDemo -> %c%n", charResult);

        short shortResult = InstanceShortMethodDemo("instanceShortMethodToCall", SIG_TMPL.formatted("S"), Z, B, C, S, I, J, F, D, L);
        System.out.printf("InstanceShortMethodDemo -> %d%n", shortResult);

        int intResult = InstanceIntMethodDemo("instanceIntMethodToCall", SIG_TMPL.formatted("I"), Z, B, C, S, I, J, F, D, L);
        System.out.printf("InstanceIntMethodDemo -> %d%n", intResult);

        long longResult = InstanceLongMethodDemo("instanceLongMethodToCall", SIG_TMPL.formatted("J"), Z, B, C, S, I, J, F, D, L);
        System.out.printf("InstanceLongMethodDemo -> %d%n", longResult);

        float floatResult = InstanceFloatMethodDemo("instanceFloatMethodToCall", SIG_TMPL.formatted("F"), Z, B, C, S, I, J, F, D, L);
        System.out.printf("InstanceFloatMethodDemo -> %f%n", floatResult);

        double doubleResult = InstanceDoubleMethodDemo("instanceDoubleMethodToCall", SIG_TMPL.formatted("D"), Z, B, C, S, I, J, F, D, L);
        System.out.printf("InstanceDoubleMethodDemo -> %f%n", doubleResult);

        InstanceVoidMethodDemo("instanceVoidMethodToCall", SIG_TMPL.formatted("V"), Z, B, C, S, I, J, F, D, L);
    }

    private Object instanceObjectMethodToCall(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l) {
        System.out.printf(MSG_TMPL, "instanceObjectMethodToCall", z, b, c, s, i, j, f, d, l);
        return "I'm a result from Java!";
    }

    private boolean instanceBooleanMethodToCall(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l) {
        System.out.printf(MSG_TMPL, "instanceBooleanMethodToCall", z, b, c, s, i, j, f, d, l);
        return true;
    }

    private byte instanceByteMethodToCall(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l) {
        System.out.printf(MSG_TMPL, "instanceByteMethodToCall", z, b, c, s, i, j, f, d, l);
        return -128;
    }

    private char instanceCharMethodToCall(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l) {
        System.out.printf(MSG_TMPL, "instanceCharMethodToCall", z, b, c, s, i, j, f, d, l);
        return 'Ї';
    }

    private short instanceShortMethodToCall(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l) {
        System.out.printf(MSG_TMPL, "instanceShortMethodToCall", z, b, c, s, i, j, f, d, l);
        return -32768;
    }

    private int instanceIntMethodToCall(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l) {
        System.out.printf(MSG_TMPL, "instanceIntMethodToCall", z, b, c, s, i, j, f, d, l);
        return -2_000_000_000;
    }

    private long instanceLongMethodToCall(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l) {
        System.out.printf(MSG_TMPL, "instanceLongMethodToCall", z, b, c, s, i, j, f, d, l);
        return -9_000_000_000_000_000_000L;
    }

    private float instanceFloatMethodToCall(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l) {
        System.out.printf(MSG_TMPL, "instanceFloatMethodToCall", z, b, c, s, i, j, f, d, l);
        return 2.71f;
    }

    private double instanceDoubleMethodToCall(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l) {
        System.out.printf(MSG_TMPL, "instanceDoubleMethodToCall", z, b, c, s, i, j, f, d, l);
        return Math.E;
    }

    private void instanceVoidMethodToCall(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object l) {
        System.out.printf(MSG_TMPL, "instanceVoidMethodToCall", z, b, c, s, i, j, f, d, l);
    }
}

// ---- VirtualDispatchDemo: tests for Call<type>MethodA with interface/abstract/overridden methods ----

interface Speakable {
    String speak();
}

class BaseAnimal {
    public String sound() {
        return "generic sound";
    }
}

abstract class AbstractSpeaker extends BaseAnimal implements Speakable {
    public abstract String speak(); // explicit abstract declaration so GetMethodID can find it
}

class Puppy extends AbstractSpeaker {
    @Override
    public String speak() {
        return "woof!";
    }

    @Override
    public String sound() {
        return "bark!";
    }
}

class VirtualDispatchDemo {
    // Calls GetMethodID with the supplied declaring class (interface / abstract / parent),
    // then uses CallObjectMethodA on the concrete instance – exercising virtual dispatch.
    private native Object CallViaDeclaringClass(Object instance, Class<?> declaringClass, String methodName, String signature);

    public void runDemo() {
        System.out.println();
        System.out.println("=== Virtual Dispatch Demo ===");

        Puppy puppy = new Puppy();
        String stringSig = "()Ljava/lang/String;";

        // Test 1: method obtained via interface, invoked on concrete instance
        Object r1 = CallViaDeclaringClass(puppy, Speakable.class, "speak", stringSig);
        System.out.println("CallViaInterface: " + r1);

        // Test 2: method obtained via abstract class, invoked on concrete instance
        Object r2 = CallViaDeclaringClass(puppy, AbstractSpeaker.class, "speak", stringSig);
        System.out.println("CallViaAbstractClass: " + r2);

        // Test 3: method obtained via parent class, invoked on overriding child instance
        Object r3 = CallViaDeclaringClass(puppy, BaseAnimal.class, "sound", stringSig);
        System.out.println("CallViaParentClass: " + r3);
    }
}
