// javac --add-exports java.base/jdk.internal.misc=ALL-UNNAMED  -d . UnsafeUsage.java

package samples.jdkinternal.unsafe.trivial;

import jdk.internal.misc.Unsafe;

public class UnsafeUsage {

    public static void main(String[] args) {
        Unsafe unsafe = Unsafe.getUnsafe();

        int isBigEndian = !unsafe.isBigEndian() ? 1 : 0;
        var bytes = (byte[]) unsafe.allocateUninitializedArray(byte.class, 3);
        int bit0 = isBigEndian + bytes.length == 4 ? 1 : 0;

        String intFieldName = new String(new char[]{'f', 'i', 'e', 'l', 'd', '3'});
        long fieldOffset = unsafe.objectFieldOffset(Examinee.class, intFieldName);
        int bit1 = fieldOffset == 2 ? 1 : 0;

        Examinee examinee = new Examinee();
        boolean updated = unsafe.compareAndSetInt(examinee, fieldOffset, 30, 40);
        int bit2 = updated ? 1 : 0;
        int bit3 = examinee.field3 == 40 ? 1 : 0;

        updated = unsafe.compareAndSetInt(examinee, fieldOffset, 30, 50);
        int bit4 = updated ? 0 : 1;
        int bit5 = examinee.field3 == 40 ? 1 : 0;

        long stringFieldOffset = unsafe.objectFieldOffset(Examinee.class, "field4");
        Object stringFieldValue = unsafe.getReferenceVolatile(examinee, stringFieldOffset);
        int bit6 = "FIELD4".equals(stringFieldValue) ? 1 : 0;

        updated = unsafe.compareAndSetReference(examinee, stringFieldOffset, "FIELD4", "FIELD4_UPDATED");
        int bit7 = updated ? 1 : 0;
        int bit8 = "FIELD4_UPDATED".equals(examinee.field4) ? 1 : 0;

        updated = unsafe.compareAndSetReference(examinee, stringFieldOffset, "FIELD4", "FIELD4_UPDATED_EVEN_MORE");
        int bit9 = updated ? 0 : 1;
        int bit10 = "FIELD4_UPDATED".equals(examinee.field4) ? 1 : 0;

        int result = 0;
        result = setBit(result, 0, bit0);
        result = setBit(result, 1, bit1);
        result = setBit(result, 2, bit2);
        result = setBit(result, 3, bit3);
        result = setBit(result, 4, bit4);
        result = setBit(result, 5, bit5);
        result = setBit(result, 6, bit6);
        result = setBit(result, 7, bit7);
        result = setBit(result, 8, bit8);
        result = setBit(result, 9, bit9);
        result = setBit(result, 10, bit10);
    }

    private static int setBit(int num, int position, int value) {
        return value == 0 ? num & ~(1 << position) : num | (1 << position);
    }
}

class Examinee {
    int field1 = 10;
    int field2 = 20;
    int field3 = 30;
    String field4 = "FIELD4";
}
