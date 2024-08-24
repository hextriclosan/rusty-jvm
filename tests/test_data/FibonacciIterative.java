

public class FibonacciIterative {

    public static void main(String[] args) {
        int n = 10; // We want the 10th Fibonacci number
        int result = fibonacci(n);
    }

    private static int fibonacci(int n) {
        if (n == 0 || n == 1) {
            return n;
        }

        int prev = 0;
        int curr = 1;
        for (int i = 2; i <= n; i++) {
            int next = prev + curr;
            prev = curr;
            curr = next;
        }

        return curr;
    }
}
