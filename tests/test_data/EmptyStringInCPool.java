package samples.javacore.strings.trivial;

public class EmptyStringInCPool {
    public static void main(String[] args) {
        int result = "".length();
        System.out.println(result == 0 ? "Empty string has length 0" : "ERROR");
    }
}
