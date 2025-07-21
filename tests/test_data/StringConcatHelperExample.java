// javac --add-exports java.base/jdk.internal.reflect=ALL-UNNAMED --patch-module java.base=. -d . StringConcatHelperExample.java
// java --patch-module java.base=. java.lang.StringConcatHelperExample

// This test is put to java.lang package for calling package-private method
// It's not so nice but a bad test is better than no test
package java.lang;

// This example mimics the way StringConcatFactory uses StringConcatHelper internally.
public class StringConcatHelperExample {
    public static void main(String[] args) {
        System.out.println(concat("a", "b", "c"));
    }

    /**
     * Concatenates the given objects into a single string.
     * <p>
     * This method is a test helper that mimics the internal usage of
     * {@code StringConcatHelper}. It converts
     * each object to a string, calculates the required buffer size, and
     * constructs the final concatenated string using {@code StringConcatHelper}.
     *
     * @param objs the objects to concatenate
     * @return the concatenated string
     */
    private static String concat(Object... objs) {
        String[] toConcat = new String[objs.length];
        for (int i = 0; i < objs.length; i++) {
            toConcat[i] = StringConcatHelper.stringOf(objs[i]);
        }

        long lengthCoder = 0;
        for (int i = toConcat.length - 1; i >= 0; i--) {
            lengthCoder = StringConcatHelper.mix(lengthCoder, toConcat[i]);
        }

        byte[] buf = StringConcatHelper.newArray(lengthCoder);

        for (int i = toConcat.length - 1; i >= 0; i--) {
            lengthCoder = StringConcatHelper.prepend(lengthCoder, buf, toConcat[i]);
        }

        return StringConcatHelper.newString(buf, lengthCoder);
    }
}
