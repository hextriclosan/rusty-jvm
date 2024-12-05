package samples.javacore.doubles.trivial;

public class LongToDoubleAndBack {
    public static void main(String[] args) {
        int result = convert();
        System.out.println(result);
    }

    private static int convert() {
        long longValue = 4608238818662014491L;
        double doubleValue = 1.23456789;
        long expectedLongValue = Double.doubleToRawLongBits(doubleValue);
        double expectedDoubleValue = Double.longBitsToDouble(longValue);

        return (longValue == expectedLongValue ? 1 : 0) + (doubleValue == expectedDoubleValue ? 1 : 0);
    }
}
