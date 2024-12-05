package samples.javacore.strings.trivial;

import java.math.BigInteger;

public class ToStringAndBack {

    public static void main(String[] args) {
        boolean valueBoolean = true;
        boolean convertedBoolean = convertBoolean(valueBoolean);
        int bit0 = valueBoolean == convertedBoolean ? 1 : 0;

        byte valueByte = 127;
        byte convertedByte = convertByte(valueByte);
        int bit1 = valueByte == convertedByte ? 1 : 0;

        short valueShort = 31999;
        short convertedShort = convertShort(valueShort);
        int bit2 = valueShort == convertedShort ? 1 : 0;

        char valueChar = 'ї';
        char convertedChar = convertChar(valueChar);
        int bit3 = valueChar == convertedChar ? 1 : 0;

        int valueInt = 1_999_999_999;
        int convertedInt = convertInt(valueInt);
        int bit4 = valueInt == convertedInt ? 1 : 0;

        long valueLong = 999_999_999_999L;
        long convertedLong = convertLong(valueLong);
        int bit5 = valueLong == convertedLong ? 1 : 0;

        float valueFloat = 3.14f;
        float convertedFloat = convertFloat(valueFloat);
        int bit6 = valueFloat == convertedFloat ? 1 : 0;

        double valueDoubled = 3.14;
        double convertedDouble = convertDouble(valueDoubled);
        int bit7 = valueDoubled == convertedDouble ? 1 : 0;

//         BigInteger valueBigInteger = new BigInteger("" + Long.MAX_VALUE);
//         BigInteger convertedBigInteger = convertBigInteger(valueBigInteger);
//         int bit8 = valueBigInteger.equals(convertedBigInteger) ? 1 : 0;

        int result = 0;
        result = setBit(result, 0, bit0);
        result = setBit(result, 1, bit1);
        result = setBit(result, 2, bit2);
        result = setBit(result, 3, bit3);
        result = setBit(result, 4, bit4);
        result = setBit(result, 5, bit5);
        result = setBit(result, 6, bit6);
        result = setBit(result, 7, bit7);
//         result = setBit(result, 8, bit8);

        System.out.println(result);
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

    private static BigInteger convertBigInteger(BigInteger valueBigInteger) {
        String string = valueBigInteger.toString(); //todo: implement clone() for arrays
        return new BigInteger(string);
    }

    private static int setBit(int num, int position, int value) {
        return value == 0 ? num & ~(1 << position) : num | (1 << position);
    }
}
