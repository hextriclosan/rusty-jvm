package samples.strings.nonvalidcesu8;

public class NonValidCESU8 {
    private static final String value = "a\ud800\ud800💔\ud800b";
    public static void main() {
        System.out.println("value: " + value);
    }
}
