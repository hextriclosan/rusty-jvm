// javac --add-exports java.base/jdk.internal.misc=ALL-UNNAMED  -d . UnsafeUsage.java
// java --add-exports java.base/jdk.internal.misc=ALL-UNNAMED samples.jdkinternal.unsafe.trivial.UnsafeUsage
package samples.jdkinternal.unsafe.trivial;

import jdk.internal.misc.Unsafe;

import java.util.Arrays;

public class UnsafeUsage {

    public static void main(String[] args) {
        Unsafe unsafe = Unsafe.getUnsafe();

        int isBigEndian = unsafe.isBigEndian() ? 1 : 0;
        System.out.print("isBigEndian: ");
        System.out.println(isBigEndian);

        var bytes = (byte[]) unsafe.allocateUninitializedArray(byte.class, 3);
        System.out.print("bytes: ");
        System.out.println(Arrays.toString(bytes));

        String intFieldName = new String(new char[]{'f', 'i', 'e', 'l', 'd', '3'});
        long intFieldOffset = unsafe.objectFieldOffset(Examinee.class, intFieldName);

        Examinee examinee = new Examinee();
        Object intFieldValue = unsafe.getIntVolatile(examinee, intFieldOffset);
        System.out.print("examinee.field3 value got by offset is: ");
        System.out.println(intFieldValue);

        boolean updated = unsafe.compareAndSetInt(examinee, intFieldOffset, 30, 40);
        if (updated) {
            System.out.print("examinee.field3 updated by offset: ");
            System.out.println(examinee.field3); // 40
        }

        updated = unsafe.compareAndSetInt(examinee, intFieldOffset, 30, 50);
        if (!updated) {
            System.out.print("examinee.field3 was not updated: ");
            System.out.println(examinee.field3); // 40
        }

        long stringFieldOffset = unsafe.objectFieldOffset(Examinee.class, "field4");
        Object stringFieldValue = unsafe.getReferenceVolatile(examinee, stringFieldOffset);
        System.out.print("examinee.field4 value got by offset is: ");
        System.out.println(stringFieldValue);

        updated = unsafe.compareAndSetReference(examinee, stringFieldOffset, "FIELD4", "FIELD4_UPDATED");
        if (updated) {
            System.out.print("examinee.field4 updated by offset: ");
            System.out.println(examinee.field4); // FIELD4_UPDATED
        }

        updated = unsafe.compareAndSetReference(examinee, stringFieldOffset, "FIELD4", "FIELD4_UPDATED_EVEN_MORE");
        if (!updated) {
            System.out.print("examinee.field4 was not updated: ");
            System.out.println(examinee.field4); // FIELD4_UPDATED
        }

        long longFieldOffset = unsafe.objectFieldOffset(Examinee.class, "field5");
        long longFieldValue = unsafe.getLongVolatile(examinee, longFieldOffset);
        System.out.print("examinee.field5 value got by offset is: ");
        System.out.println(longFieldValue);

        updated = unsafe.compareAndSetLong(examinee, longFieldOffset, 42_949_672_980L, 128_849_018_920L);
        if (updated) {
            System.out.print("examinee.field5 updated by offset: ");
            System.out.println(examinee.field5); // 128_849_018_920L
        }

        updated = unsafe.compareAndSetLong(examinee, longFieldOffset, 42_949_672_980L, 1L);
        if (!updated) {
            System.out.print("examinee.field5 was not updated: ");
            System.out.println(examinee.field5); // 128_849_018_920L
        }

        Examinee one = new Examinee();
        Examinee two = new Examinee();
        Examinee three = new Examinee();
        Examinee[] examinees = new Examinee[]{one, two, three};
        int index = 1;
        int arrayBaseOffset = unsafe.arrayBaseOffset(Examinee[].class);
        int scale = unsafe.arrayIndexScale(Examinee[].class);
        if ((scale & (scale - 1)) != 0) {
            throw new RuntimeException("array index scale not a power of two");
        }
        int arrayShift = 31 - Integer.numberOfLeadingZeros(scale);
        long offset = ((long) index << arrayShift) + arrayBaseOffset;
        Examinee item = (Examinee) unsafe.getReferenceAcquire(examinees, offset);
        System.out.print("examinees[1] got by offset is `two`: ");
        System.out.println(item == two);

        updated = unsafe.compareAndSetReference(examinees, offset, two, three);
        if (updated) {
            System.out.print("examinees[1] updated by offset and set to `three`: ");
            System.out.println(examinees[1] == three);
        }

        updated = unsafe.compareAndSetReference(examinees, offset, two, one);
        if (!updated) {
            System.out.print("examinees[1] was not updated and remains the same: ");
            System.out.println(examinees[1] == three);
        }

        unsafe.putReferenceVolatile(one, stringFieldOffset, "FIELD4_PUT_REFERENCE_VOLATILE");
        System.out.print("one.field4 updated by offset to: ");
        System.out.println(one.field4); // FIELD4_PUT_REFERENCE_VOLATILE
    }
}

class Examinee {
    int field1 = 10;
    int field2 = 20;
    int field3 = 30;
    String field4 = "FIELD4";
    long field5 = 42_949_672_980L/*h=10,l=20*/;
}
