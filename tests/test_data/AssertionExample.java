package samples.javacore.assertions.trivial;

public class AssertionExample {
    public static void main(String[] args) {
        System.out.print("Assertions: ");
        assert assertionEnabled();
    }

    private static boolean assertionEnabled() {
        System.out.println("enabled");
        return true;
    }
}
