package samples.jni;

public class JniVariadicDemo {
    static { System.loadLibrary("jni_test_lib"); }
    private static native int verify(Object receiver, Class<?> base);

    public static class Base {
        public static int voidCalls;
        public int marker;
        public Base() {}
        public Base(int value) {
            if (value < 0) { throw new IllegalArgumentException("constructor"); }
            marker = value;
        }
        public int zero() { return 9; }
        public static int staticZero() { return 10; }
        public int throwing() { throw new IllegalArgumentException("method"); }
        public static int staticThrowing() { throw new IllegalArgumentException("static"); }
        public Base(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object object, int i1, int i2, int i3, int i4, int i5, double d1, double d2, double d3, double d4, double d5, double d6, double d7, double d8) { validate(z, b, c, s, i, j, f, d, object, i1, i2, i3, i4, i5, d1, d2, d3, d4, d5, d6, d7, d8); marker = 42; }
        public static void validate(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object object, int i1, int i2, int i3, int i4, int i5, double d1, double d2, double d3, double d4, double d5, double d6, double d7, double d8) {
            if (!z || b != -101 || c != 0xabcd || s != -23456 || i != 123456
                    || j != 0x1122334455667788L || f != 1.25f || d != -2.5 || object == null
                    || i1 != 1 || i2 != 2 || i3 != 3 || i4 != 4 || i5 != 5
                    || d1 != 1.0 || d2 != 2.0 || d3 != 3.0 || d4 != 4.0
                    || d5 != 5.0 || d6 != 6.0 || d7 != 7.0 || d8 != 8.0) {
                throw new AssertionError("variadic arguments");
            }
        }
        public Object valueObject(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object object, int i1, int i2, int i3, int i4, int i5, double d1, double d2, double d3, double d4, double d5, double d6, double d7, double d8) { validate(z, b, c, s, i, j, f, d, object, i1, i2, i3, i4, i5, d1, d2, d3, d4, d5, d6, d7, d8); return object; }
        public static Object staticObject(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object object, int i1, int i2, int i3, int i4, int i5, double d1, double d2, double d3, double d4, double d5, double d6, double d7, double d8) { validate(z, b, c, s, i, j, f, d, object, i1, i2, i3, i4, i5, d1, d2, d3, d4, d5, d6, d7, d8); return object; }
        public boolean valueBoolean(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object object, int i1, int i2, int i3, int i4, int i5, double d1, double d2, double d3, double d4, double d5, double d6, double d7, double d8) { validate(z, b, c, s, i, j, f, d, object, i1, i2, i3, i4, i5, d1, d2, d3, d4, d5, d6, d7, d8); return true; }
        public static boolean staticBoolean(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object object, int i1, int i2, int i3, int i4, int i5, double d1, double d2, double d3, double d4, double d5, double d6, double d7, double d8) { validate(z, b, c, s, i, j, f, d, object, i1, i2, i3, i4, i5, d1, d2, d3, d4, d5, d6, d7, d8); return true; }
        public byte valueByte(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object object, int i1, int i2, int i3, int i4, int i5, double d1, double d2, double d3, double d4, double d5, double d6, double d7, double d8) { validate(z, b, c, s, i, j, f, d, object, i1, i2, i3, i4, i5, d1, d2, d3, d4, d5, d6, d7, d8); return (byte)-101; }
        public static byte staticByte(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object object, int i1, int i2, int i3, int i4, int i5, double d1, double d2, double d3, double d4, double d5, double d6, double d7, double d8) { validate(z, b, c, s, i, j, f, d, object, i1, i2, i3, i4, i5, d1, d2, d3, d4, d5, d6, d7, d8); return (byte)-101; }
        public char valueChar(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object object, int i1, int i2, int i3, int i4, int i5, double d1, double d2, double d3, double d4, double d5, double d6, double d7, double d8) { validate(z, b, c, s, i, j, f, d, object, i1, i2, i3, i4, i5, d1, d2, d3, d4, d5, d6, d7, d8); return (char)0xabcd; }
        public static char staticChar(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object object, int i1, int i2, int i3, int i4, int i5, double d1, double d2, double d3, double d4, double d5, double d6, double d7, double d8) { validate(z, b, c, s, i, j, f, d, object, i1, i2, i3, i4, i5, d1, d2, d3, d4, d5, d6, d7, d8); return (char)0xabcd; }
        public short valueShort(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object object, int i1, int i2, int i3, int i4, int i5, double d1, double d2, double d3, double d4, double d5, double d6, double d7, double d8) { validate(z, b, c, s, i, j, f, d, object, i1, i2, i3, i4, i5, d1, d2, d3, d4, d5, d6, d7, d8); return (short)-23456; }
        public static short staticShort(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object object, int i1, int i2, int i3, int i4, int i5, double d1, double d2, double d3, double d4, double d5, double d6, double d7, double d8) { validate(z, b, c, s, i, j, f, d, object, i1, i2, i3, i4, i5, d1, d2, d3, d4, d5, d6, d7, d8); return (short)-23456; }
        public int valueInt(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object object, int i1, int i2, int i3, int i4, int i5, double d1, double d2, double d3, double d4, double d5, double d6, double d7, double d8) { validate(z, b, c, s, i, j, f, d, object, i1, i2, i3, i4, i5, d1, d2, d3, d4, d5, d6, d7, d8); return 123456; }
        public static int staticInt(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object object, int i1, int i2, int i3, int i4, int i5, double d1, double d2, double d3, double d4, double d5, double d6, double d7, double d8) { validate(z, b, c, s, i, j, f, d, object, i1, i2, i3, i4, i5, d1, d2, d3, d4, d5, d6, d7, d8); return 123456; }
        public long valueLong(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object object, int i1, int i2, int i3, int i4, int i5, double d1, double d2, double d3, double d4, double d5, double d6, double d7, double d8) { validate(z, b, c, s, i, j, f, d, object, i1, i2, i3, i4, i5, d1, d2, d3, d4, d5, d6, d7, d8); return 0x1122334455667788L; }
        public static long staticLong(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object object, int i1, int i2, int i3, int i4, int i5, double d1, double d2, double d3, double d4, double d5, double d6, double d7, double d8) { validate(z, b, c, s, i, j, f, d, object, i1, i2, i3, i4, i5, d1, d2, d3, d4, d5, d6, d7, d8); return 0x1122334455667788L; }
        public float valueFloat(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object object, int i1, int i2, int i3, int i4, int i5, double d1, double d2, double d3, double d4, double d5, double d6, double d7, double d8) { validate(z, b, c, s, i, j, f, d, object, i1, i2, i3, i4, i5, d1, d2, d3, d4, d5, d6, d7, d8); return 1.25f; }
        public static float staticFloat(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object object, int i1, int i2, int i3, int i4, int i5, double d1, double d2, double d3, double d4, double d5, double d6, double d7, double d8) { validate(z, b, c, s, i, j, f, d, object, i1, i2, i3, i4, i5, d1, d2, d3, d4, d5, d6, d7, d8); return 1.25f; }
        public double valueDouble(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object object, int i1, int i2, int i3, int i4, int i5, double d1, double d2, double d3, double d4, double d5, double d6, double d7, double d8) { validate(z, b, c, s, i, j, f, d, object, i1, i2, i3, i4, i5, d1, d2, d3, d4, d5, d6, d7, d8); return -2.5; }
        public static double staticDouble(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object object, int i1, int i2, int i3, int i4, int i5, double d1, double d2, double d3, double d4, double d5, double d6, double d7, double d8) { validate(z, b, c, s, i, j, f, d, object, i1, i2, i3, i4, i5, d1, d2, d3, d4, d5, d6, d7, d8); return -2.5; }
        public void valueVoid(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object object, int i1, int i2, int i3, int i4, int i5, double d1, double d2, double d3, double d4, double d5, double d6, double d7, double d8) { validate(z, b, c, s, i, j, f, d, object, i1, i2, i3, i4, i5, d1, d2, d3, d4, d5, d6, d7, d8); voidCalls++; }
        public static void staticVoid(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object object, int i1, int i2, int i3, int i4, int i5, double d1, double d2, double d3, double d4, double d5, double d6, double d7, double d8) { validate(z, b, c, s, i, j, f, d, object, i1, i2, i3, i4, i5, d1, d2, d3, d4, d5, d6, d7, d8); voidCalls++; }
    }
    public static class Child extends Base {
        @Override public int valueInt(boolean z, byte b, char c, short s, int i, long j, float f, double d, Object object, int i1, int i2, int i3, int i4, int i5, double d1, double d2, double d3, double d4, double d5, double d6, double d7, double d8) { validate(z, b, c, s, i, j, f, d, object, i1, i2, i3, i4, i5, d1, d2, d3, d4, d5, d6, d7, d8); return 654321; }
        @Override public int zero() { return 11; }
    }
    public static void main(String[] args) {
        int result = verify(new Child(), Base.class);
        if (result != 0) { throw new AssertionError("JNI check " + result); }
        System.out.println("variadic-ok");
    }
}
