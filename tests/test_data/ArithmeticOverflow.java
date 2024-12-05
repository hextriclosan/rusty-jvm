package samples.arithmetics.overflow;

public class ArithmeticOverflow {
    public static void main(String[] args) {
        int intsResult = intsOverflow();
        int longsResult = longsOverflow();

        boolean result = intsResult == 511 && longsResult == 511;
        System.out.println(result ? 1 : 0);
    }

    private static int intsOverflow() {
        // Add
        int addOverflow = getAddOverflow(1);
        int bit0 = addOverflow == -2147483648 ? 1 : 0;

        // Multiply
        int mulOverflow = getMulOverflow(2);
        int bit1 = mulOverflow == -2 ? 1 : 0;

        // Division
        int divOverflow = getDivOverflow(-1); // Causes overflow
        int bit2 = divOverflow == -2147483648 ? 1 : 0;

        // Negation
        int negOverflow = getNegOverflow(Integer.MIN_VALUE); // Integer.MIN_VALUE has no positive counterpart
        int bit3 = negOverflow == -2147483648 ? 1 : 0;

        // Remainder
        int remOverflow = getRemOverflow(-1); // Same as division overflow
        int bit4 = remOverflow == 0 ? 1 : 0;

        // Shift Left
        int shlOverflow = getShlOverflow(1); // Shifting may cause bits to overflow
        int bit5 = shlOverflow == -2 ? 1 : 0;

        // Shift Right
        int shrOverflow = getShrOverflow(1); // Arithmetic shift retains sign, but no overflow
        int bit6 = shrOverflow == -1073741824 ? 1 : 0;

        int uShlOverflow = getUShlOverflow(1);// Logical shift retains sign, but no overflow
        int bit7 = uShlOverflow == 1073741824 ? 1 : 0;

        // Subtract
        int subOverflow = getSubOverflow(1);
        int bit8 = subOverflow == 2147483647 ? 1 : 0;

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
        return result;
    }

    private static int longsOverflow() {
        // Add
        long addOverflow = getAddOverflow(1L);
        int bit0 = addOverflow == Long.MIN_VALUE ? 1 : 0;

        // Multiply
        long mulOverflow = getMulOverflow(2L);
        int bit1 = mulOverflow == -2L ? 1 : 0;

        // Division
        long divOverflow = getDivOverflow(-1L); // Causes overflow
        int bit2 = divOverflow == Long.MIN_VALUE ? 1 : 0;

        // Negation
        long negOverflow = getNegOverflow(Long.MIN_VALUE); // Long.MIN_VALUE has no positive counterpart
        int bit3 = negOverflow == Long.MIN_VALUE ? 1 : 0;

        // Remainder
        long remOverflow = getRemOverflow(-1L); // Same as division overflow
        int bit4 = remOverflow == 0L ? 1 : 0;

        // Shift Left
        long shlOverflow = getShlOverflow(1L); // Shifting may cause bits to overflow
        int bit5 = shlOverflow == -2L ? 1 : 0;

        // Shift Right
        long shrOverflow = getShrOverflow(1L); // Arithmetic shift retains sign, but no overflow
        int bit6 = shrOverflow == -4611686018427387904L ? 1 : 0;

        long uShlOverflow = getUShlOverflow(1L); // Logical shift retains sign, but no overflow
        int bit7 = uShlOverflow == 4611686018427387904L ? 1 : 0;

        // Subtract
        long subOverflow = getSubOverflow(1L);
        int bit8 = subOverflow == Long.MAX_VALUE ? 1 : 0;

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
        return result;
    }

    private static int getSubOverflow(int value) {
        return Integer.MIN_VALUE - value;
    }

    private static int getShrOverflow(int value) {
        return Integer.MIN_VALUE >> value;
    }

    private static int getShlOverflow(int value) {
        return Integer.MAX_VALUE << value;
    }

    private static int getUShlOverflow(int value) {
        return Integer.MIN_VALUE >>> value;
    }

    private static int getRemOverflow(int value) {
        return Integer.MIN_VALUE % value;
    }

    private static int getNegOverflow(int value) {
        return -value;
    }

    private static int getDivOverflow(int value) {
        return Integer.MIN_VALUE / value;
    }

    private static int getMulOverflow(int value) {
        return Integer.MAX_VALUE * value;
    }

    private static int getAddOverflow(int value) {
        return Integer.MAX_VALUE + value;
    }

    private static long getSubOverflow(long value) {
        return Long.MIN_VALUE - value;
    }

    private static long getShrOverflow(long value) {
        return Long.MIN_VALUE >> value;
    }

    private static long getShlOverflow(long value) {
        return Long.MAX_VALUE << value;
    }

    private static long getUShlOverflow(long value) {
        return Long.MIN_VALUE >>> value;
    }

    private static long getRemOverflow(long value) {
        return Long.MIN_VALUE % value;
    }

    private static long getNegOverflow(long value) {
        return -value;
    }

    private static long getDivOverflow(long value) {
        return Long.MIN_VALUE / value;
    }

    private static long getMulOverflow(long value) {
        return Long.MAX_VALUE * value;
    }

    private static long getAddOverflow(long value) {
        return Long.MAX_VALUE + value;
    }

    private static int setBit(int num, int position, int value) {
        return value == 0 ? num & ~(1 << position) : num | (1 << position);
    }
}
