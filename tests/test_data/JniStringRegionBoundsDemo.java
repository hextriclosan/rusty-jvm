package samples.javacore.loadlibrary.stringops;

public class JniStringRegionBoundsDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native String get(String string, int start, int length);
    private static native byte[] getUtf(String string, int start, int length);
    private static native void getNullBuffer(String string, int start, int length);
    private static native void getUtfNullBuffer(String string, int start, int length);

    public static void main(String[] args) {
        run("get in bounds", () -> get("value", 1, 3));
        run("get empty at length", () -> get("value", 5, 0));
        run("get negative start", () -> get("value", -1, 1));
        run("get negative length", () -> get("value", 0, -1));
        run("get past end", () -> get("value", 3, 3));
        run("get overflow", () -> get("value", 1, Integer.MAX_VALUE));
        run("utf in bounds", () -> hex(getUtf("value", 1, 3)));
        run("utf non-ascii", () -> hex(getUtf("a\u00e9\u20ac\uD83D\uDE00z", 1, 4)));
        run("utf split surrogate", () -> hex(getUtf("a\uD83D\uDE00z", 1, 1)));
        run("utf nul char", () -> hex(getUtf("a\u0000b", 0, 3)));
        run("utf empty at length", () -> hex(getUtf("value", 5, 0)));
        run("utf negative start", () -> hex(getUtf("value", -1, 1)));
        run("utf past end", () -> hex(getUtf("value", 3, 3)));
        run("utf overflow", () -> hex(getUtf("value", 1, Integer.MAX_VALUE)));
        run("get null buffer empty", () -> { getNullBuffer("value", 0, 0); return "done"; });
        run("utf null buffer empty", () -> { getUtfNullBuffer("value", 0, 0); return "done"; });
        run("get null string", () -> get(null, 0, 1));
        run("utf null string", () -> hex(getUtf(null, 0, 1)));
        run("get null buffer", () -> { getNullBuffer("value", 0, 1); return "done"; });
        run("utf null buffer", () -> { getUtfNullBuffer("value", 0, 1); return "done"; });
    }

    private interface Operation {
        String run();
    }

    private static void run(String label, Operation operation) {
        try {
            System.out.println(label + " = " + escape(operation.run()));
        } catch (RuntimeException exception) {
            System.out.println(label + " -> " + exception);
        }
    }

    private static String hex(byte[] bytes) {
        StringBuilder builder = new StringBuilder();
        for (byte b : bytes) {
            if (builder.length() > 0) {
                builder.append(' ');
            }
            builder.append(String.format("%02x", b & 0xff));
        }
        return builder.toString();
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
