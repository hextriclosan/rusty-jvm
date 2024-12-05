package samples.javacore.bytes.trivial;

public class ByteOperations {
    public static void main(String[] args) {
        int[] bits = new int[21];

        // Trivial case: Byte declaration and assignment
        byte b1 = -10;
        bits[0] = b1 == -10 ? 1 : 0;

        // Simple arithmetic operations
        byte b2 = 20;
        byte sum = (byte) (b1 + b2);
        bits[1] = sum == 10 ? 1 : 0;

        // Comparison
        bits[2] = b1 < b2 ? 1 : 0;

        // Increment and decrement
        b1++;
        bits[3] = b1 == -9 ? 1 : 0;
        b1--;
        bits[4] = b1 == -10 ? 1 : 0;
        b2++;
        bits[5] = b2 == 21 ? 1 : 0;
        b2--;
        bits[6] = b2 == 20 ? 1 : 0;

        // Overflow and underflow
        byte bMax = Byte.MAX_VALUE; // 127
        byte bMin = Byte.MIN_VALUE; // -128
        bits[7] = bMax == 127 ? 1 : 0;
        bits[8] = bMin == -128 ? 1 : 0;

        // Overflow
        byte overflow = (byte) (bMax + 1); // should wrap to -128
        bits[9] = overflow == -128 ? 1 : 0;

        // Underflow
        byte underflow = (byte) (bMin - 1); // should wrap to 127
        bits[10] = underflow == 127 ? 1 : 0;

        // Casting higher types to byte (narrowing)
        int largerValue = 130; // beyond byte range
        byte narrowedValue = (byte) largerValue; // overflow occurs
        bits[11] = narrowedValue == -126 ? 1 : 0;

        // Bitwise operations (AND, OR, XOR, NOT)
        byte bits1 = 0b0101; // binary 5
        byte bits2 = 0b0011; // binary 3

        byte andResult = (byte) (bits1 & bits2);
        byte orResult = (byte) (bits1 | bits2);
        byte xorResult = (byte) (bits1 ^ bits2);
        byte notResult = (byte) ~bits1;

        bits[12] = andResult == 1 ? 1 : 0;
        bits[13] = orResult == 7 ? 1 : 0;
        bits[14] = xorResult == 6 ? 1 : 0;
        bits[15] = notResult == -6 ? 1 : 0;

        // Shifting operations
        byte bShift = 8; // 0b00001000
        byte shiftedLeft = (byte) (bShift << 1); // 16
        bits[16] = shiftedLeft == 16 ? 1 : 0;

        byte shiftedRight = (byte) (bShift >> 1); // 16
        bits[17] = shiftedRight == 4 ? 1 : 0;

        byte shiftedRightUnsigned = (byte) (bShift >>> 1); // 16
        bits[18] = shiftedRightUnsigned == 4 ? 1 : 0;

        // Tricky: Handling negative numbers with shifts
        byte negativeByte = -8;
        int shiftedRightOfNegative = negativeByte >> 1;
        bits[19] = shiftedRightOfNegative == -4 ? 1 : 0;

        // large positive value due to sign bit preservation
        int shiftedRightUnsignedOfNegative = negativeByte >>> 1;
        bits[20] = shiftedRightUnsignedOfNegative == 2147483644 ? 1 : 0;

        int result = 0;
        for (int i = 0; i < bits.length; i++) {
            result = setBit(result, i, bits[i]);
        }
        int finalResult = result;
        System.out.println(finalResult);
    }

    private static int setBit(int num, int position, int value) {
        return value == 0 ? num & ~(1 << position) : num | (1 << position);
    }
}
