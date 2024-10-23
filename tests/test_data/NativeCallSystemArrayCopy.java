package samples.nativecall.system;

import java.util.Map;
import java.util.HashMap;

public class NativeCallSystemArrayCopy {
    public static void main(String[] args) {
        var props = new HashMap<String, String>();
        props.put("java.class.version", "67.0"); //  fixme: move to VM init stage
        jdk.internal.misc.VM.saveProperties(props); // javac --add-exports java.base/jdk.internal.misc=ALL-UNNAMED  -d . NativeCallSystemArrayCopy.java

        long sum = intArr() + longArr() + intArrOverlapping() + fromBaseArray();
    }

    private static int intArr() {
        int[] src = {10, 20, 30, 40, 50, 60};
        int[] dest = {1, 2, 3, 4, 5, 6};

        System.arraycopy(src, 1, dest, 2, 3);

        int sum = 0;
        for (int i : dest) {
            sum += i;
        }
        return sum;
    }

    private static long longArr() {
        long[] src = {
                42_949_672_980L/*h=10,l=20*/,
                128_849_018_920L/*h=30,l=40*/,
                214_748_364_860L/*h=50,l=60*/,
                300_647_710_800L/*h=70,l=80*/,
                386_547_056_740L/*h=90,l=100*/,
                472_446_402_680L/*h=110,l=120*/
        };
        long[] dest = {1, 2, 3, 4, 5, 6};

        System.arraycopy(src, 1, dest, 2, 3);

        long sum = 0;
        for (long i : dest) {
            sum += i;
        }
        return sum;
    }

    private static int intArrOverlapping() {
        int[] arr = {10, 20, 30, 40, 50, 60};

        System.arraycopy(arr, 0, arr, 1, 5);

        int sum = 0;
        for (int i : arr) {
            sum += i;
        }
        return sum;
    }

    private static int fromBaseArray() {
        Integer[] src = {10, 20, 30};
        Number[] dest = new Number[3];

        System.arraycopy(src, 0, dest, 0, dest.length);

        int sum = 0;
        for (Number number : dest) {
            sum += number.intValue();
        }
        return sum;
    }
}
