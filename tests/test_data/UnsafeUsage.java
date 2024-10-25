// javac --add-exports java.base/jdk.internal.misc=ALL-UNNAMED  -d . UnsafeUsage.java

package samples.jdkinternal.unsafe.trivial;

import jdk.internal.misc.Unsafe;

public class UnsafeUsage {

    public static void main(String[] args) {
        Unsafe unsafe = Unsafe.getUnsafe();

        int isBigEndian = !unsafe.isBigEndian() ? 1 : 0;
        var bytes = (byte[]) unsafe.allocateUninitializedArray(byte.class, 3);

        int result = isBigEndian + bytes.length;
    }

}
