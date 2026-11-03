package samples.javacore.loadlibrary.stringops;

public class JniModifiedUtf8Demo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native String newStringUtf(byte[] bytes);
    private static native byte[] getStringUtfChars(String string);
    private static native int getStringUtfLength(String string);
    private static native long getStringUtfLengthAsLong(String string);

    public static void main(String[] args) {
        decode("empty");
        decode("ascii", 0x76, 0x61, 0x6c, 0x75, 0x65);
        decode("two bytes", 0xc3, 0xa9);
        decode("three bytes", 0xe2, 0x82, 0xac);
        decode("surrogate pair", 0xed, 0xa0, 0xbd, 0xed, 0xb8, 0x80);
        decode("lone high surrogate", 0x61, 0xed, 0xa0, 0xbd, 0x62);
        decode("lone low surrogate", 0x61, 0xed, 0xb8, 0x80, 0x62);
        decode("reversed surrogates", 0xed, 0xb8, 0x80, 0xed, 0xa0, 0xbd);
        decode("modified nul", 0x61, 0xc0, 0x80, 0x62);
        decode("standard four bytes", 0xf0, 0x9f, 0x98, 0x80);
        decode("truncated three bytes", 0xe2, 0x82);
        decode("lone continuation", 0x61, 0x80, 0x62);
        decode("bad continuation", 0xc3, 0x41);

        // Built from chars at runtime, since lone surrogates may not survive in string constants
        encode("empty");
        encode("ascii", 'v', 'a', 'l', 'u', 'e');
        encode("two bytes", 0xe9);
        encode("three bytes", 0x20ac);
        encode("surrogate pair", 0xd83d, 0xde00);
        encode("lone high surrogate", 'a', 0xd83d, 'b');
        encode("lone low surrogate", 'a', 0xde00, 'b');
        encode("reversed surrogates", 0xde00, 0xd83d);
        encode("modified nul", 'a', 0, 'b');
    }

    private static void decode(String label, int... values) {
        byte[] bytes = new byte[values.length];
        for (int i = 0; i < values.length; i++) {
            bytes[i] = (byte) values[i];
        }
        System.out.println("decode " + label + " = " + escape(newStringUtf(bytes)));
    }

    private static void encode(String label, int... values) {
        char[] chars = new char[values.length];
        for (int i = 0; i < values.length; i++) {
            chars[i] = (char) values[i];
        }
        String string = new String(chars);
        System.out.println("encode " + label + " = " + hex(getStringUtfChars(string))
                + " length=" + getStringUtfLength(string) + "/" + getStringUtfLengthAsLong(string));
    }

    private static String hex(byte[] bytes) {
        StringBuilder builder = new StringBuilder("[");
        for (byte b : bytes) {
            if (builder.length() > 1) {
                builder.append(' ');
            }
            builder.append(String.format("%02x", b & 0xff));
        }
        return builder.append(']').toString();
    }

    private static String escape(String value) {
        StringBuilder builder = new StringBuilder("\"");
        for (char c : value.toCharArray()) {
            if (c < 0x20 || c > 0x7e) {
                builder.append(String.format("\\u%04x", (int) c));
            } else {
                builder.append(c);
            }
        }
        return builder.append('"').toString();
    }
}
