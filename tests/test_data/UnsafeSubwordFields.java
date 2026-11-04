package samples.jdkinternal.unsafe.subwordfields;

import java.lang.reflect.Field;
import jdk.internal.misc.Unsafe;

public class UnsafeSubwordFields {
    private static final Unsafe U = Unsafe.getUnsafe();

    public static void main(String[] args) throws Exception {
        Fields fields = new Fields();
        System.out.println("byte=" + U.getByte(fields, offset("byteValue")));
        System.out.println("short=" + U.getShort(fields, offset("shortValue")));
        System.out.println("char=" + (int) U.getChar(fields, offset("charValue")));
        System.out.println("int=" + U.getInt(fields, offset("intValue")));
        System.out.println("long=" + U.getLong(fields, offset("longValue")));
        System.out.println("reference=" + U.getReference(fields, offset("referenceValue")));

        U.putByte(fields, offset("byteValue"), (byte) 127);
        U.putShort(fields, offset("shortValue"), (short) -32768);
        U.putChar(fields, offset("charValue"), '￿');
        U.putInt(fields, offset("intValue"), Integer.MIN_VALUE);
        U.putLong(fields, offset("longValue"), Long.MAX_VALUE);
        U.putReference(fields, offset("referenceValue"), "put");
        System.out.println("after put: " + fields.byteValue + " " + fields.shortValue + " "
                + (int) fields.charValue + " " + fields.intValue + " " + fields.longValue + " "
                + fields.referenceValue);

        // Static fields are addressed through the base and offset Unsafe gives for them
        Object base = U.staticFieldBase(Fields.class.getDeclaredField("staticByteValue"));
        System.out.println("static byte=" + U.getByte(base, staticOffset("staticByteValue")));
        System.out.println("static short=" + U.getShort(base, staticOffset("staticShortValue")));
        System.out.println("static char=" + (int) U.getChar(base, staticOffset("staticCharValue")));
        System.out.println("static int=" + U.getInt(base, staticOffset("staticIntValue")));
        System.out.println("static long=" + U.getLong(base, staticOffset("staticLongValue")));
        System.out.println("static reference=" + U.getReference(base, staticOffset("staticReferenceValue")));

        U.putByte(base, staticOffset("staticByteValue"), (byte) -1);
        U.putShort(base, staticOffset("staticShortValue"), (short) 32767);
        U.putChar(base, staticOffset("staticCharValue"), 'A');
        U.putInt(base, staticOffset("staticIntValue"), Integer.MAX_VALUE);
        U.putLong(base, staticOffset("staticLongValue"), Long.MIN_VALUE);
        U.putReference(base, staticOffset("staticReferenceValue"), "static put");
        System.out.println("after static put: " + Fields.staticByteValue + " " + Fields.staticShortValue + " "
                + (int) Fields.staticCharValue + " " + Fields.staticIntValue + " " + Fields.staticLongValue + " "
                + Fields.staticReferenceValue);
    }

    private static long offset(String name) {
        return U.objectFieldOffset(Fields.class, name);
    }

    private static long staticOffset(String name) throws NoSuchFieldException {
        Field field = Fields.class.getDeclaredField(name);
        return U.staticFieldOffset(field);
    }
}

class Fields {
    byte byteValue = -7;
    short shortValue = 1234;
    char charValue = 'Я';
    int intValue = 123456789;
    long longValue = -1234567890123L;
    Object referenceValue = "instance";

    static byte staticByteValue = 9;
    static short staticShortValue = -300;
    static char staticCharValue = '€';
    static int staticIntValue = -42;
    static long staticLongValue = 9876543210L;
    static Object staticReferenceValue = "static";
}
