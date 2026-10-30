// javac --add-exports java.base/jdk.internal.misc=ALL-UNNAMED -d . UnsafeObjectFieldOffsetErrors.java
// java --add-exports java.base/jdk.internal.misc=ALL-UNNAMED samples.jdkinternal.unsafe.objectfieldoffseterrors.UnsafeObjectFieldOffsetErrors

package samples.jdkinternal.unsafe.objectfieldoffseterrors;

import jdk.internal.misc.Unsafe;

// Unsafe.objectFieldOffset(Class, String) for fields it doesn't hand out an instance offset for.
// The outcome differs between JDK releases, so the test compares it with the reference JVM.
public class UnsafeObjectFieldOffsetErrors {
    private static int staticField;
    private int instanceField;

    public static void main(String[] args) {
        Unsafe unsafe = Unsafe.getUnsafe();
        for (String name : new String[] {"instanceField", "staticField", "missingField"}) {
            try {
                long offset = unsafe.objectFieldOffset(UnsafeObjectFieldOffsetErrors.class, name);
                // offsets are VM-specific, only their validity is comparable
                System.out.println(name + ": offset " + (offset >= 0 ? "is valid" : offset));
            } catch (InternalError e) {
                System.out.println(name + ": " + e);
            }
        }
    }
}
