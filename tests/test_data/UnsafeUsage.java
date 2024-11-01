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

        int result = 0;
        result = setBit(result, 0, bit0);
        result = setBit(result, 1, bit1);
    }

    private static int setBit(int num, int position, int value) {
        return value == 0 ? num & ~(1 << position) : num | (1 << position);
    }
}

class Examinee {
    private int field1 = 10;
    private int field2 = 20;
    private int field3 = 30;
}
