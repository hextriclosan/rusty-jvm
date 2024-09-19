
public class SubLong {
    private static long first;
    private static long second;

    static {
        first = 100_000_000_000L;
        second = 99_000_000_000L;
    }

    public static void main(String[] args) {
        long result = second - first;
    }
}
