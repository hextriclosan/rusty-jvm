package samples.staticinit.interfaceinitializationdoesnotinitializesuperinterfaces;

public class InterfaceInitializationDoesNotInitializeSuperinterfaces {
    public static void main(String[] args) {
        System.out.println("main starts");
        System.out.println(J.i);
        System.out.println(K.j);
        System.out.println("main ends");
    }
}

// Example 12.4.1-3. Interface Initialization Does Not Initialize Superinterfaces
interface I {
    int i = 1, ii = Test.out("ii", 2);
}

interface J extends I {
    int j = Test.out("j", 3), jj = Test.out("jj", 4);
}

interface K extends J {
    int k = Test.out("k", 5);
}

class Test {
    static int out(String s, int i) {
        System.out.println(s + "=" + i);
        return i;
    }
}
