package samples.staticinit.parentstaticfield;

public class OnlyTheClassThatDeclaresStaticFieldIsInitialized {
    public static void main(String[] args) {
        System.out.println("main starts");
        System.out.println(Sub.taxi);
        System.out.println("main ends");
    }
}

// JLS: Example 12.4.1-2. Only The Class That Declares static Field Is Initialized
class Super {
    static int taxi = 1729;
}

class Sub extends Super {
    static {
        System.out.print("Sub ");
    }
}
