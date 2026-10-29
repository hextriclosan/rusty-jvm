package samples.reflection.nativeaccessorreturnvalues;

import java.lang.reflect.Method;

// Run with -Djdk.reflect.useNativeAccessorOnly=true so Method.invoke goes through the VM's
// NativeAccessor.invoke0, which has to box primitive results.
public class NativeAccessorReturnValues {
    static boolean aBoolean() { return true; }
    static boolean aFalse() { return false; }
    static byte aByte() { return -7; }
    static char aChar() { return 'x'; }
    static short aShort() { return 1234; }
    static int anInt() { return 42; }
    static long aLong() { return 1L << 40; }
    static float aFloat() { return 1.5f; }
    static double aDouble() { return -2.25; }
    static String aString() { return "str"; }
    static void aVoid() { }

    private final int base = 10;
    long instanceLong(int add) { return base + add + (1L << 33); }

    public static void main(String[] args) throws Exception {
        String[] names = {"aBoolean", "aFalse", "aByte", "aChar", "aShort", "anInt", "aLong",
                "aFloat", "aDouble", "aString", "aVoid"};
        for (String name : names) {
            Method method = NativeAccessorReturnValues.class.getDeclaredMethod(name);
            Object result = method.invoke(null);
            System.out.println(name + " -> " + result
                    + (result == null ? "" : " (" + result.getClass().getSimpleName() + ")"));
        }
        Method instanceLong = NativeAccessorReturnValues.class.getDeclaredMethod("instanceLong", int.class);
        System.out.println("instanceLong -> " + instanceLong.invoke(new NativeAccessorReturnValues(), 5));
    }
}
