package samples.nativecall.system;

public class NativeCallSystemArrayCopy {
    public static void main(String[] args) {
        int bit0 = intArr() == 99 ? 1 : 0;
        int bit1 = longArr() == 644245094589L ? 1 : 0;
        int bit2 = intArrOverlapping() == 160 ? 1 : 0;
        int bit3 = fromBaseArray() == 60 ? 1 : 0;

        int result = 0;
        result = setBit(result, 0, bit0);
        result = setBit(result, 1, bit1);
        result = setBit(result, 2, bit2);
        result = setBit(result, 3, bit3);
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

    private static int setBit(int num, int position, int value) {
        return value == 0 ? num & ~(1 << position) : num | (1 << position);
    }
}
