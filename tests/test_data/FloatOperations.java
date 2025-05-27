package samples.arithmetics.operations.floats;

public class FloatOperations {
    public static void main(String[] args) {
        // Large and small values
        float large = 1.989e30f;      // Mass of the Sun (kg)
        float small = 1.8549e-43f;    // Planck time (s)
        float tiny = 1.4E-45f;        // Smallest positive float
        float huge = 3.4028235e38f;   // Largest float
        double oneDouble = 1.0;       // Exact double value
        double hugeDouble = 1.7e308;  // Large double

        // Arithmetic operations
        float sum = large + huge;               // Addition with large numbers
        float diff = huge - large;              // Subtraction with large numbers
        float product = small * tiny;           // Multiplication with small numbers
        float quotient = huge / large;          // Division with large numbers
        float remainder = huge % large;         // Modulus operation
        float remainderOfZero = 0f % 0f;        // Modulus operation with zero
        float negLarge = -large;                // Negation of a large number
        float convertedExact = (float) oneDouble;     // Conversion from double to float
        float convertedOverflow = (float) hugeDouble; // Conversion from double to float with overflow


        // Special cases
        float underflow = small / large;        // Underflow: tiny result
        float overflow = huge * huge;           // Overflow: exceeds max float
        float nan = 0.0f / 0.0f;                // NaN (Not a Number)

        System.out.println(sum);
        System.out.println(diff);
        System.out.println(product);
        System.out.println(quotient);
        System.out.println(remainder);
        System.out.println(remainderOfZero);
        System.out.println(negLarge);
        System.out.println(convertedExact);
        System.out.println(convertedOverflow);
        System.out.println(underflow);
        System.out.println(overflow);
        System.out.println(nan);
    }
}
