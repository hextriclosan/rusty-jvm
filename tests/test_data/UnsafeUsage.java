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

        long longFieldOffset = unsafe.objectFieldOffset(Examinee.class, "field5");
        updated = unsafe.compareAndSetLong(examinee, longFieldOffset, 42_949_672_980L, 128_849_018_920L);
        int bit11 = updated ? 1 : 0;
        int bit12 = examinee.field5 == 128_849_018_920L ? 1 : 0;

        updated = unsafe.compareAndSetLong(examinee, longFieldOffset, 42_949_672_980L, 1L);
        int bit13 = updated ? 0 : 1;
        int bit14 = examinee.field5 == 128_849_018_920L ? 1 : 0;

        Examinee one = new Examinee();
        Examinee two = new Examinee();
        Examinee three = new Examinee();
        Examinee[] examinees = new Examinee[] {one, two, three};
        int index = 1;
        int arrayBaseOffset = unsafe.arrayBaseOffset(Examinee[].class);
        int scale = unsafe.arrayIndexScale(Examinee[].class);
        if ((scale & (scale - 1)) != 0) {
            throw new RuntimeException("array index scale not a power of two");
        }
        int arrayShift = 31 - Integer.numberOfLeadingZeros(scale);
        long offset = ((long)index << arrayShift) + arrayBaseOffset;
        Examinee item = (Examinee)unsafe.getReferenceAcquire(examinees, offset);
        int bit15 = item == two ? 1 : 0;
        updated = unsafe.compareAndSetReference(examinees, offset, two, three);
        int bit16 = updated ? 1 : 0;
        int bit17 = examinees[1] == three ? 1 : 0;
        updated = unsafe.compareAndSetReference(examinees, offset, two, one);
        int bit18 = updated ? 0 : 1;
        int bit19 = examinees[1] == three ? 1 : 0;

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
        result = setBit(result, 11, bit11);
        result = setBit(result, 12, bit12);
        result = setBit(result, 13, bit13);
        result = setBit(result, 14, bit14);
        result = setBit(result, 15, bit15);
        result = setBit(result, 16, bit16);
        result = setBit(result, 17, bit17);
        result = setBit(result, 18, bit18);
        result = setBit(result, 19, bit19);
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
    long field5 = 42_949_672_980L/*h=10,l=20*/;
}
