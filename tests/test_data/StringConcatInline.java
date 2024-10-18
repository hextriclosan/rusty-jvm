// javac -XDstringConcat=inline  -d . StringConcatInline.java

package samples.javacore.strings.concat.trivial;

public class StringConcatInline {
    public static void main(String[] args) {
        int result = concat();
    }

    private static int concat() {
        String abc = "abc";
        String def = "def😂";
        String abcdef = abc + def;

        int sum = 0;
        for (char c : abcdef.toCharArray()) {
            sum += c;
        }

        return sum;
    }
}
