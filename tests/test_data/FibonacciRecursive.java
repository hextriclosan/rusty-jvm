package samples.arithmetics.fibonacci.recursive;

public class FibonacciRecursive {

    public static void main(String[] args) {
        int n = 10; // We want the 10th Fibonacci number
        int result = fibonacci(n);
    }

    private static int fibonacci(int n) {
        if (n <= 1) {
            return n;
        }
        return fibonacci(n - 1) + fibonacci(n - 2);
    }

}
