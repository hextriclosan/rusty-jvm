// build command:
// javac -g -parameters Trivial.java

public class Trivial<T> implements Runnable {
    public static final float PI = 3.14159265f;
    protected static final int SPEED_OF_LIGHT = 299792458;
    private static final int MIN_INT = -2147483648;
    private static final long MIN_LONG = -9223372036854775808L;
    private static final long MAX_LONG = 9223372036854775807L;
    private static final double MAX_DOUBLE = 1.7976931348623157e+308;
    private static final double MIN_DOUBLE = 4.9E-324;

    protected String someText;

    public Trivial(String someText) {
        this.someText = someText;
    }

    public Trivial() {
        this(null);
    }

    public int add(int first, final int second) throws ClassNotFoundException {
        int result = first + second;

        return result > 0 ? result : 0;
    }

    @Override
    public void run() {
        Runnable runnable = () -> {};
        interface LocalCls {
        }
    }

    class InnerCls {
    }
}
