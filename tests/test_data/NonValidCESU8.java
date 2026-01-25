package samples.javacore.strings.nonvalidcesu8;

public class NonValidCESU8 {
    private static final String value = "a\ud800\ud800💔\ud800b";
    public static void main(String[] args) {
        System.out.println("value: " + value);
    }
}
