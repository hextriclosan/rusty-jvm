// javac --add-exports java.base/jdk.internal.reflect=ALL-UNNAMED --patch-module java.base=. -d . StringConcatHelperExample.java
// java --patch-module java.base=. java.lang.StringConcatHelperExample

// This test is put to java.lang package for calling package-private method
// It's not so nice but a bad test is better than no test
package java.lang;

// This example mimics the way StringConcatFactory uses StringConcatHelper internally.
// Only entry points that are stable across JDK releases are used here: the lower-level
// mix/prepend/newString helpers they delegate to change their signatures between releases.
public class StringConcatHelperExample {
    public static void main(String[] args) {
        System.out.println(concat("a", "b", "c"));
    }

    /**
     * Concatenates the given objects into a single string.
     * <p>
     * This method is a test helper that mimics the internal usage of
     * {@code StringConcatHelper}. It converts each object to a string and
     * folds them together using {@code StringConcatHelper.simpleConcat}.
     *
     * @param objs the objects to concatenate
     * @return the concatenated string
     */
    private static String concat(Object... objs) {
        String result = "";
        for (Object obj : objs) {
            result = StringConcatHelper.simpleConcat(result, StringConcatHelper.stringOf(obj));
        }
        return result;
    }
}
