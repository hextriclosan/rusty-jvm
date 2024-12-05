package samples.arithmetics.extremestack.ints;

public class ExtremeStackInt {

    public static void main(String[] args) {
        // Test nested operations and method calls
        int nestedResult = nestedCalculations(10);
        //System.out.println("nestedResult=" + nestedResult);

        // Test loop operations with complex conditions
        int loopResult = complexLoop(5);
        //System.out.println("loopResult=" + loopResult);

        // Test combined recursive methods
        int combinedRecursionResult = factorialPlusFibonacci(5);
        //System.out.println("combinedRecursionResult=" + combinedRecursionResult);

        int result = nestedResult + loopResult + combinedRecursionResult; // 528
        System.out.println(result);
    }

    // Method with nested operations and method calls
    private static int nestedCalculations(int n) {
        int result = 0;
        for (int i = 0; i < n; i++) {
            result += multiplyAndAddAndBitAndUShiftRight(i, n - i);
        }
        return result * 2 - n; // Final operation after loop
    }

    // Helper method for nestedCalculations
    private static int multiplyAndAddAndBitAndUShiftRight(int x, int y) {
        return (x * y) + (x - y) + (y & x) + (y >>> x); // Simple arithmetic operations
    }

    // Method with loop, conditions, and nested operations
    private static int complexLoop(int n) {
        int result = 1;
        for (int i = 1; i <= n; i++) {
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
    private static int factorialPlusFibonacci(int n) {
        return factorial(n) + fibonacci(n);
    }

    // Recursive method to calculate factorial
    private static int factorial(int n) {
        if (n <= 1) {
            return 1;
        }
        return n * factorial(n - 1);
    }

    // Recursive method to calculate the nth Fibonacci number
    private static int fibonacci(int n) {
        if (n <= 1) {
            return n;
        }
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
