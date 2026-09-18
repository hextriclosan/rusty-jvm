package samples.jdkinternal.unsafe.subwordfields;

import jdk.internal.misc.Unsafe;

public class UnsafeSubwordFields {
    private static final Unsafe U = Unsafe.getUnsafe();

    public static void main(String[] args) throws Exception {
        Fields fields = new Fields();
        System.out.println("byte=" + U.getByte(fields, U.objectFieldOffset(Fields.class, "byteValue")));
        System.out.println("short=" + U.getShort(fields, U.objectFieldOffset(Fields.class, "shortValue")));
        System.out.println("char=" + U.getChar(fields, U.objectFieldOffset(Fields.class, "charValue")));
        var staticField = Fields.class.getDeclaredField("staticByteValue");
        System.out.println("static byte=" + U.getByte(
                U.staticFieldBase(staticField), U.staticFieldOffset(staticField)));
    }
}

class Fields {
    byte byteValue = -7;
    short shortValue = 1234;
    char charValue = 'Я';
    static byte staticByteValue = 9;
}
