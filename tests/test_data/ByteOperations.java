package samples.javacore.bytes.trivial;

public class ByteOperations {
    public static void main(String[] args) {
        // Trivial case: Byte declaration and assignment
        byte b1 = -10;
        System.out.print("b1: ");
        System.out.println(b1);

        // Simple arithmetic operations
        byte b2 = 20;
        byte sum = (byte) (b1 + b2);
        System.out.print("sum: ");
        System.out.println(sum);

        // Comparison
        System.out.print("b1 < b2: ");
        System.out.println(b1 < b2);

        // Increment and decrement
        b1++;
        System.out.print("b1++: ");
        System.out.println(b1);

        b1--;
        System.out.print("b1--: ");
        System.out.println(b1);

        b2++;
        System.out.print("b2++: ");
        System.out.println(b2);
        b2--;
        System.out.print("b2--: ");
        System.out.println(b2);

        // Overflow and underflow
        byte bMax = Byte.MAX_VALUE; // 127
        byte bMin = Byte.MIN_VALUE; // -128
        System.out.print("bMax: ");
        System.out.println(bMax);
        System.out.print("bMin: ");
        System.out.println(bMin);

        // Overflow
        byte overflow = (byte) (bMax + 1); // should wrap to -128
        System.out.print("overflow: ");
        System.out.println(overflow);

        // Underflow
        byte underflow = (byte) (bMin - 1); // should wrap to 127
        System.out.print("underflow: ");
        System.out.println(underflow);

        // Casting higher types to byte (narrowing)
        int largerValue = 130; // beyond byte range
        byte narrowedValue = (byte) largerValue; // overflow occurs
        System.out.print("narrowedValue: ");
        System.out.println(narrowedValue); // -126

        // Bitwise operations (AND, OR, XOR, NOT)
        byte bits1 = 0b0101; // binary 5
        byte bits2 = 0b0011; // binary 3

        byte andResult = (byte) (bits1 & bits2); // 0b0001 = 1
        byte orResult = (byte) (bits1 | bits2); // 0b0111 = 7
        byte xorResult = (byte) (bits1 ^ bits2); // 0b0110 = 6
        byte notResult = (byte) ~bits1; // 0b11111010 = -6
        System.out.print("andResult: ");
        System.out.println(andResult);
        System.out.print("orResult: ");
        System.out.println(orResult);
        System.out.print("xorResult: ");
        System.out.println(xorResult);
        System.out.print("notResult: ");
        System.out.println(notResult);

        // Shifting operations
        byte bShift = 8; // 0b00001000
        byte shiftedLeft = (byte) (bShift << 1); // 16
        System.out.print("shiftedLeft: ");
        System.out.println(shiftedLeft);

        byte shiftedRight = (byte) (bShift >> 1); // 4
        System.out.print("shiftedRight: ");
        System.out.println(shiftedRight);

        byte shiftedRightUnsigned = (byte) (bShift >>> 1); // 4
        System.out.print("shiftedRightUnsigned: ");
        System.out.println(shiftedRightUnsigned);

        // Tricky: Handling negative numbers with shifts
        byte negativeByte = -8;
        int shiftedRightOfNegative = negativeByte >> 1; // -4
        System.out.print("shiftedRightOfNegative: ");
        System.out.println(shiftedRightOfNegative);

        // large positive value due to sign bit preservation
        int shiftedRightUnsignedOfNegative = negativeByte >>> 1; // 2147483644
        System.out.print("shiftedRightUnsignedOfNegative: ");
        System.out.println(shiftedRightUnsignedOfNegative);
    }
}
