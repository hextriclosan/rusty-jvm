package samples.staticfield.interfacehierarchy;

public class StaticFieldFromSecondParentInterfaceExample {
    public static void main(String[] args) {
        // Combined extends FirstParent (no field) and SecondParent (has VALUE).
        // Since VALUE is initialized with a non-constant expression, javac emits
        // `getstatic Combined VALUE`, forcing runtime lookup through the interface
        // hierarchy. The lookup must skip FirstParent (no field) and find the field
        // in SecondParent - exercising the `continue` branch.
        System.out.println(Combined.VALUE);
    }
}

interface FirstParent {
    // no fields
}

interface SecondParent {
    int VALUE = Initializer.init(42);
}

interface Combined extends FirstParent, SecondParent {
    // no direct fields; VALUE is inherited from SecondParent via FirstParent skip
}

// Helper class to initialize the static field with a non-constant expression.
// This forces javac to emit a `getstatic` instruction for VALUE, which in turn forces the JVM
// to perform a runtime lookup through the interface hierarchy.
class Initializer {
    static int init(int value) {
        return value;
    }
}
