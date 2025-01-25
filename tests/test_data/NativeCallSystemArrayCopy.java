package samples.nativecall.system;

import java.util.Arrays;

public class NativeCallSystemArrayCopy {
    public static void main(String[] args) {
        intArr();
        longArr();
        intArrOverlapping();
        longArrOverlapping();
        upcasting();
        // downcasting(); https://github.com/hextriclosan/rusty-jvm/issues/175, https://github.com/hextriclosan/rusty-jvm/issues/176
    }

    private static void intArr() {
        int[] src = {10, 20, 30, 40, 50, 60};
        int[] dest = {1, 2, 3, 4, 5, 6};

        System.arraycopy(src, 1, dest, 2, 3);
        System.out.print("intArr(): ");
        System.out.println(Arrays.toString(dest));
    }

    private static void longArr() {
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
        System.out.print("longArr(): ");
        System.out.println(Arrays.toString(dest));
    }

    private static void intArrOverlapping() {
        int[] arr = {10, 20, 30, 40, 50, 60};

        System.arraycopy(arr, 0, arr, 1, 5);
        System.out.print("intArrOverlapping(): ");
        System.out.println(Arrays.toString(arr));
    }

    private static void longArrOverlapping() {
        long[] arr = {
                42_949_672_980L/*h=10,l=20*/,
                128_849_018_920L/*h=30,l=40*/,
                214_748_364_860L/*h=50,l=60*/,
                300_647_710_800L/*h=70,l=80*/,
                386_547_056_740L/*h=90,l=100*/,
                472_446_402_680L/*h=110,l=120*/};

        System.arraycopy(arr, 0, arr, 1, 5);
        System.out.print("longArrOverlapping(): ");
        System.out.println(Arrays.toString(arr));
    }

    private static void upcasting() {
        Integer[] src = {10, 20, 30};
        Number[] dest = new Number[3];

        System.arraycopy(src, 0, dest, 0, dest.length);
        System.out.print("upcasting(): ");
        System.out.println(Arrays.toString(dest));
    }

    private static void downcasting() {
        System.out.print("downcasting(): ");
        try {
            Double[] dest = {3.14, 2.71, 1.41};
            Number[] src = {1, 2, 3};
            System.arraycopy(src, 0, dest, 0, dest.length);
            System.out.println(Arrays.toString(dest));
        } catch (ArrayStoreException e) {
            System.out.println(e.getMessage());
        }
    }
}
