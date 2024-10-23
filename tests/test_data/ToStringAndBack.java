package samples.javacore.strings.trivial;

public class ToStringAndBack {

    public static void main(String[] args) {
        boolean convertedBoolean = convertBoolean();
        byte convertedByte = convertByte();
        short convertedShort = convertShort();
        char convertedChar = convertChar();
        int convertedInt = convertInt();
        long convertedLong = convertLong();
        float convertedFloat = convertFloat();
        double convertedDouble = 0;//convertDouble(); fixme: freezes after WIDE ops

        long result = (convertedBoolean ? 1 : 0) + convertedByte + convertedShort + convertedChar + convertedInt + convertedLong + (long) convertedFloat + (long) convertedDouble;
    }

    private static boolean convertBoolean() {
        boolean value = true;
        String string = Boolean.toString(value);
        return Boolean.parseBoolean(string);
    }

    private static byte convertByte() {
        byte value = 127;
        String string = Byte.toString(value);
        return Byte.parseByte(string);
    }

    private static short convertShort() {
        short value = 31999;
        String string = Short.toString(value);
        return Short.parseShort(string);
    }

    private static char convertChar() {
        char value = 'ї';
        String string = Character.toString(value);
        return string.charAt(0);
    }

    private static int convertInt() {
        int value = 1_999_999_999;
        String string = Integer.toString(value);
        return Integer.parseInt(string);
    }

    private static long convertLong() {
        long value = 999_999_999_999L;
        String string = Long.toString(value);
        return Long.parseLong(string);
    }

    private static double convertDouble() {
        double value = 3.14;
        String string = Double.toString(value);
        return Double.parseDouble(string);
    }

    private static float convertFloat() {
        float value = 3.14f;
        String string = Float.toString(value);
        return Float.parseFloat(string);
    }
}
