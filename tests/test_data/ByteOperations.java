package samples.javacore.bytes.trivial;

public class ByteOperations {
    public static void main(String[] args) {
        // Trivial case: Byte declaration and assignment
        byte b1 = -10;
        System.out.println("b1: " + b1);

        // Simple arithmetic operations
        byte b2 = 20;
        byte sum = (byte) (b1 + b2);
        System.out.println("sum: " + sum);

        // Comparison
        System.out.println("b1 < b2: " + (b1 < b2));

        // Increment and decrement
        b1++;
        System.out.println("b1++: " + b1);

        b1--;
        System.out.println("b1--: " + b1);

        b2++;
        System.out.println("b2++: " + b2);
        b2--;
        System.out.println("b2--: " + b2);

        // Overflow and underflow
        byte bMax = Byte.MAX_VALUE; // 127
        byte bMin = Byte.MIN_VALUE; // -128
        System.out.println("bMax: " + bMax);
        System.out.println("bMin: " + bMin);

        // Overflow
        byte overflow = (byte) (bMax + 1); // should wrap to -128
        System.out.println("overflow: " + overflow);

        // Underflow
        byte underflow = (byte) (bMin - 1); // should wrap to 127
        System.out.println("underflow: " + underflow);

        // Casting higher types to byte (narrowing)
        int largerValue = 130; // beyond byte range
        byte narrowedValue = (byte) largerValue; // overflow occurs
        System.out.println("narrowedValue: " + narrowedValue); // -126

        // Bitwise operations (AND, OR, XOR, NOT)
        byte bits1 = 0b0101; // binary 5
        byte bits2 = 0b0011; // binary 3

        byte andResult = (byte) (bits1 & bits2); // 0b0001 = 1
        byte orResult = (byte) (bits1 | bits2); // 0b0111 = 7
        byte xorResult = (byte) (bits1 ^ bits2); // 0b0110 = 6
        byte notResult = (byte) ~bits1; // 0b11111010 = -6
        System.out.println("andResult: " + andResult);
        System.out.println("orResult: " + orResult);
        System.out.println("xorResult: " + xorResult);
        System.out.println("notResult: " + notResult);

        // Shifting operations
        byte bShift = 8; // 0b00001000
        byte shiftedLeft = (byte) (bShift << 1); // 16
        System.out.println("shiftedLeft: " + shiftedLeft);

        byte shiftedRight = (byte) (bShift >> 1); // 4
        System.out.println("shiftedRight: " + shiftedRight);

        byte shiftedRightUnsigned = (byte) (bShift >>> 1); // 4
        System.out.println("shiftedRightUnsigned: " + shiftedRightUnsigned);

        // Tricky: Handling negative numbers with shifts
        byte negativeByte = -8;
        int shiftedRightOfNegative = negativeByte >> 1; // -4
        System.out.println("shiftedRightOfNegative: " + shiftedRightOfNegative);

        // large positive value due to sign bit preservation
        int shiftedRightUnsignedOfNegative = negativeByte >>> 1; // 2147483644
        System.out.println("shiftedRightUnsignedOfNegative: " + shiftedRightUnsignedOfNegative);
    }
}
