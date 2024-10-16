package samples.arithmetics.operations.doubles;

public class DoubleOperations {
    public static void main(String[] args) {
        // Large and small values
        double large = 1.989e30;     // Mass of the Sun (kg)
        double small = 1.8549e-43;   // Planck time (s)
        double tiny = 5e-324;        // Smallest positive double
        double huge = 1.7e308;       // Largest double

        // Arithmetic operations
        double sum = large + huge;               // Addition with large numbers
        double diff = huge - large;              // Subtraction with large numbers
        double product = small * tiny;           // Multiplication with small numbers
        double quotient = huge / large;          // Division with large numbers
        double remainder = huge % large;         // Modulus operation


        // Special cases
        double underflow = small / large;        // Underflow: tiny result
        double overflow = huge * huge;           // Overflow: exceeds max double
        double nan = 0.0 / 0.0;                  // NaN (Not a Number)

        double result = sum/1e100 + diff/1e100 + product + quotient + remainder + underflow +
                (overflow == Double.POSITIVE_INFINITY ? 1e278 : 0) +
                (Double.isNaN(nan) ? 1e278 : 0);
    }
}