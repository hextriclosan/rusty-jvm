package samples.arithmetics.extremestack.longs;

public class ExtremeStackLong {

    public static void main(String[] args) {
        // Test nested operations and method calls
        long nestedResult = nestedCalculations(10);
        //System.out.println("nestedResult=" + nestedResult);

        // Test loop operations with complex conditions
        long loopResult = complexLoop(5);
        //System.out.println("loopResult=" + loopResult);

        // Test combined recursive methods
        long combinedRecursionResult = factorialPlusFibonacci(5);
        //System.out.println("combinedRecursionResult=" + combinedRecursionResult);

        long result = nestedResult + loopResult + combinedRecursionResult;
        //System.out.println("result=" + result);
    }

    // Method with nested operations and method calls
    private static long nestedCalculations(long n) {
        long result = 0;
        for (long i = 0; i < n; i++) {
            result += multiplyAndAdd(i, n - i);
        }
        return result * 2 - n; // Final operation after loop
    }

    // Helper method for nestedCalculations
    private static long multiplyAndAdd(long x, long y) {
        return (x * y) + (x - y); // Simple arithmetic operations
    }

    // Method with loop, conditions, and nested operations
    private static long complexLoop(long n) {
        long result = 1;
        for (long i = 1; i <= n; i++) {
            if (i % 2 == 0) {
                result *= i; // Multiply for even numbers
            } else {
                result += i; // Add for odd numbers
            }
            result -= (i % 3 == 0) ? 1 : 0; // Subtract 1 if divisible by 3
        }
        return result;
    }

    // Method combining recursion from factorial and Fibonacci
    private static long factorialPlusFibonacci(long n) {
        return factorial(n) + fibonacci(n);
    }

    // Recursive method to calculate factorial
    private static long factorial(long n) {
        if (n <= 1) {
            return 1;
        }
        return n * factorial(n - 1);
    }

    // Recursive method to calculate the nth Fibonacci number
    private static long fibonacci(long n) {
        if (n <= 1) {
            return n;
        }
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
