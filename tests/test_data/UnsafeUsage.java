// javac --add-exports java.base/jdk.internal.misc=ALL-UNNAMED -d . UnsafeUsage.java
// java --add-exports java.base/jdk.internal.misc=ALL-UNNAMED samples.jdkinternal.unsafe.trivial.UnsafeUsage
package samples.jdkinternal.unsafe.trivial;

import jdk.internal.misc.Unsafe;

import java.lang.reflect.Field;
import java.util.Arrays;

public class UnsafeUsage {
    private static final Unsafe U = Unsafe.getUnsafe();

    public static void main(String[] args) throws Exception {
        isBigEndian();
        allocateUninitializedArray();
        compareAndSetInt();
        compareAndSetReference();
        compareAndSetLong();
        compareAndExchangeLong();
        getReferenceAcquire();
        modifyClassFieldValue();
        getSetStaticField();
        putInt();
        putIntVolatile();
        putLong();
        setGetByteArray();
        setGetShortArray();
        setGetCharArray();
        setGetIntArray();
        setGetLongArray();
    }

    private static void isBigEndian() {
        int isBigEndian = U.isBigEndian() ? 1 : 0;
        System.out.println("isBigEndian: " + isBigEndian);
    }

    private static void allocateUninitializedArray() {
        var bytes = (byte[]) U.allocateUninitializedArray(byte.class, 3);
        System.out.println("bytes: " + Arrays.toString(bytes));
    }

    private static void compareAndSetInt() {
        String intFieldName = new String(new char[]{'f', 'i', 'e', 'l', 'd', '3'});
        long intFieldOffset = U.objectFieldOffset(Examinee.class, intFieldName);

        Examinee examinee = new Examinee();
        Object intFieldValue = U.getIntVolatile(examinee, intFieldOffset);
        System.out.println("examinee.field3 value got by offset is: " + intFieldValue);

        boolean updated = U.compareAndSetInt(examinee, intFieldOffset, 30, 40);
        if (updated) {
            System.out.println("examinee.field3 updated by offset: " + examinee.field3); // 40
        }

        updated = U.compareAndSetInt(examinee, intFieldOffset, 30, 50);
        if (!updated) {
            System.out.println("examinee.field3 was not updated: " + examinee.field3); // 40
        }
    }

    private static void compareAndSetLong() {
        long longFieldOffset = U.objectFieldOffset(Examinee.class, "field5");
        Examinee examinee = new Examinee();
        long longFieldValue = U.getLongVolatile(examinee, longFieldOffset);
        System.out.println("examinee.field5 value got by offset is: " + longFieldValue);

        boolean updated = U.compareAndSetLong(examinee, longFieldOffset, 42_949_672_980L, 128_849_018_920L);
        System.out.println("compareAndSetLong on field examinee.field5: updated=" + updated + " currentValue=" + examinee.field5);

        updated = U.compareAndSetLong(examinee, longFieldOffset, 42_949_672_980L, 1L);
        System.out.println("compareAndSetLong on field examinee.field5: updated=" + updated + " currentValue=" + examinee.field5);
    }

    private static void compareAndExchangeLong() {
        long longFieldOffset = U.objectFieldOffset(Examinee.class, "field5");
        Examinee examinee = new Examinee();
        long longFieldValue = U.getLongVolatile(examinee, longFieldOffset);
        System.out.println("examinee.field5 value got by offset is: " + longFieldValue);

        long oldValue = U.compareAndExchangeLong(examinee, longFieldOffset, 42_949_672_980L, 128_849_018_920L);
        System.out.println("compareAndExchangeLong on field examinee.field5: oldValue=" + oldValue + " currentValue=" + examinee.field5);


        oldValue = U.compareAndExchangeLong(examinee, longFieldOffset, 42_949_672_980L, 1L);
        System.out.println("compareAndExchangeLong on field examinee.field5: oldValue=" + oldValue + " currentValue=" + examinee.field5);
    }

    private static void compareAndSetReference() {
        long stringFieldOffset = U.objectFieldOffset(Examinee.class, "field4");
        Examinee examinee = new Examinee();
        Object stringFieldValue = U.getReferenceVolatile(examinee, stringFieldOffset);
        System.out.println("examinee.field4 value got by offset is: " + stringFieldValue);

        boolean updated = U.compareAndSetReference(examinee, stringFieldOffset, "FIELD4", "FIELD4_UPDATED");
        if (updated) {
            System.out.println("examinee.field4 updated by offset: " + examinee.field4); // FIELD4_UPDATED
        }

        updated = U.compareAndSetReference(examinee, stringFieldOffset, "FIELD4", "FIELD4_UPDATED_EVEN_MORE");
        if (!updated) {
            System.out.println("examinee.field4 was not updated: " + examinee.field4); // FIELD4_UPDATED
        }
    }

    private static void getReferenceAcquire() {
        Examinee one = new Examinee();
        Examinee two = new Examinee();
        Examinee three = new Examinee();
        Examinee[] examinees = new Examinee[]{one, two, three};
        long offset = getArrayOffset(Examinee[].class, 1);
        Examinee item = (Examinee) U.getReferenceAcquire(examinees, offset);
        System.out.println("examinees[1] got by offset is `two`: " + (item == two));

        boolean updated = U.compareAndSetReference(examinees, offset, two, three);
        if (updated) {
            System.out.println("examinees[1] updated by offset and set to `three`: " + (examinees[1] == three));
        }

        updated = U.compareAndSetReference(examinees, offset, two, one);
        if (!updated) {
            System.out.println("examinees[1] was not updated and remains the same: " + (examinees[1] == three));
        }

        long stringFieldOffset = U.objectFieldOffset(Examinee.class, "field4");
        U.putReferenceVolatile(one, stringFieldOffset, "FIELD4_PUT_REFERENCE_VOLATILE");
        System.out.println("one.field4 updated by offset to: " + one.field4); // FIELD4_PUT_REFERENCE_VOLATILE
    }

    private static void modifyClassFieldValue() throws NoSuchFieldException {
        Class<?> classAsInstance = Integer.class;
        String originalName = classAsInstance.getName();
        String newName = "java.lang.Positron";
        Field nameField = Class.class.getDeclaredField("name");
        long fieldOffset = U.objectFieldOffset(nameField);
        boolean updated = U.compareAndSetReference(classAsInstance, fieldOffset, originalName, newName);

        if (updated) {
            System.out.println("Class<Integer>.name update new value is " + Integer.class.getName());

            // IMPORTANT: Restore the original name to keep the JVM stable.
            U.putReference(classAsInstance, fieldOffset, originalName);
            System.out.println("State restored. Name is now: " + Integer.class.getName());
        } else {
            System.out.println("FAILURE: Could not modify Class<Integer>.name.");
        }
    }

    private static void getSetStaticField() throws NoSuchFieldException {
        Field staticField = Examinee.class.getDeclaredField("staticField");
        System.out.println("Examinee.staticField as java.lang.reflect.Field: " + staticField);
        long staticFieldOffset = U.staticFieldOffset(staticField);
        Object staticFieldBase = U.staticFieldBase(staticField);
        System.out.println("staticFieldBase: " + staticFieldBase);

        Object originalStaticValue = U.getReference(staticFieldBase, staticFieldOffset);
        System.out.println("Current static value: " + originalStaticValue);
        U.putReference(staticFieldBase, staticFieldOffset, "staticFieldNewValue");
        System.out.println("New value set via putReference(...): " + Examinee.staticField);

        U.putReference(staticFieldBase, staticFieldOffset, originalStaticValue);
        System.out.println("State restored. Static value is now: " + Examinee.staticField);
    }

    private static void putInt() {
        long intFieldOffset = U.objectFieldOffset(Examinee.class, "field3");
        Examinee examinee = new Examinee();
        int intFieldValue = U.getIntVolatile(examinee, intFieldOffset);
        System.out.println("examinee.field3 value got by offset is: " + intFieldValue);

        U.putInt(examinee, intFieldOffset, 1337);
        System.out.println("putInt on field examinee.field3: currentValue=" + examinee.field3);
    }

    private static void putIntVolatile() {
        long intFieldOffset = U.objectFieldOffset(Examinee.class, "field3");
        Examinee examinee = new Examinee();
        int intFieldValue = U.getIntVolatile(examinee, intFieldOffset);
        System.out.println("examinee.field3 value got by offset is: " + intFieldValue);

        U.putIntVolatile(examinee, intFieldOffset, 1337);
        System.out.println("putIntVolatile on field examinee.field3: currentValue=" + examinee.field3);
    }

    private static void putLong() {
        long longFieldOffset = U.objectFieldOffset(Examinee.class, "field5");
        Examinee examinee = new Examinee();
        long longFieldValue = U.getLongVolatile(examinee, longFieldOffset);
        System.out.println("examinee.field5 value got by offset is: " + longFieldValue);

        U.putLong(examinee, longFieldOffset, 128_849_018_920L);
        System.out.println("putLong on field examinee.field5: currentValue=" + examinee.field5);
    }

    private static void setGetByteArray() {
        long offset = getArrayOffset(byte[].class, 5);
        byte[] bytes = new byte[11];
        U.putByte(bytes, offset, (byte) 100);
        System.out.println(Arrays.toString(bytes));
        byte b = U.getByte(bytes, offset);
        System.out.println("byte at index 5 is: " + b);
    }

    private static void setGetShortArray() {
        long offset = getArrayOffset(short[].class, 5);
        short[] shorts = new short[11];
        U.putShort(shorts, offset, (short) 10000);
        System.out.println(Arrays.toString(shorts));
        short s = U.getShort(shorts, offset);
        System.out.println("short at index 5 is: " + s);
    }

    private static void setGetCharArray() {
        long offset = getArrayOffset(char[].class, 5);
        char[] chars = new char[]{'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'};
        U.putChar(chars, offset, 'b');
        System.out.println(Arrays.toString(chars));
        char c = U.getChar(chars, offset);
        System.out.println("char at index 5 is: " + c);
    }

    private static void setGetIntArray() {
        long offset = getArrayOffset(int[].class, 5);
        int[] ints = new int[11];
        U.putInt(ints, offset, 1_000_000_000);
        System.out.println(Arrays.toString(ints));
        int i = U.getInt(ints, offset);
        System.out.println("int at index 5 is: " + i);
    }

    private static void setGetLongArray() {
        long offset = getArrayOffset(long[].class, 5);
        long[] longs = new long[11];
        U.putLong(longs, offset, 1_000_000_000_000L);
        System.out.println(Arrays.toString(longs));
        long l = U.getLong(longs, offset);
        System.out.println("long at index 5 is: " + l);
    }

    private static long getArrayOffset(Class<?> clazz, int index) {
        long arrayBaseOffset = U.arrayBaseOffset(clazz);
        int scale = U.arrayIndexScale(clazz);
        if ((scale & (scale - 1)) != 0) {
            throw new RuntimeException("array index scale not a power of two");
        }
        int arrayShift = 31 - Integer.numberOfLeadingZeros(scale);
        long offset = ((long) index << arrayShift) + arrayBaseOffset;
        return offset;
    }
}

class Examinee {
    int field1 = 10;
    int field2 = 20;
    static String staticField = "staticFieldValue";
    int field3 = 30;
    String field4 = "FIELD4";
    long field5 = 42_949_672_980L/*h=10,l=20*/;
}
