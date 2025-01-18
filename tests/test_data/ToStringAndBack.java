package samples.javacore.strings.trivial;

import java.math.BigDecimal;
import java.math.BigInteger;

public class ToStringAndBack {

    public static void main(String[] args) {
        System.out.println(convertBoolean(true));
        System.out.println(convertByte((byte)127));
        System.out.println(convertShort((short)31999));
        System.out.println(convertChar('ї'));
        System.out.println(convertInt(1_999_999_999));
        System.out.println(convertLong(999_999_999_999L));
        System.out.println(convertFloat(3.14f));
        System.out.println(convertDouble(3.14));
        System.out.println(convertBigInteger(new BigInteger("340282366920938463463374607431768211455"))); //2^128 − 1
        System.out.println(convertBigDecimal(new BigDecimal("340282366920938463463374607431768211455.340282366920938463463374607431768211455")));
    }

    private static boolean convertBoolean(boolean value) {
        String string = Boolean.toString(value);
        return Boolean.parseBoolean(string);
    }

    private static byte convertByte(byte value) {
        String string = Byte.toString(value);
        return Byte.parseByte(string);
    }

    private static short convertShort(short value) {
        String string = Short.toString(value);
        return Short.parseShort(string);
    }

    private static char convertChar(char value) {
        String string = Character.toString(value);
        return string.charAt(0);
    }

    private static int convertInt(int value) {
        String string = Integer.toString(value);
        return Integer.parseInt(string);
    }

    private static long convertLong(long value) {
        String string = Long.toString(value);
        return Long.parseLong(string);
    }

    private static double convertDouble(double value) {
        String string = Double.toString(value);
        return Double.parseDouble(string);
    }

    private static float convertFloat(float value) {
        String string = Float.toString(value);
        return Float.parseFloat(string);
    }

    private static BigInteger convertBigInteger(BigInteger value) {
        String string = value.toString();
        return new BigInteger(string);
    }

    private static BigDecimal convertBigDecimal(BigDecimal value) {
        String string = value.toString();
        return new BigDecimal(string);
    }
}
