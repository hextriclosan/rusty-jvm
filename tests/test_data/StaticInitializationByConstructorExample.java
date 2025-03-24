package samples.staticinit.byconstructor;

public class StaticInitializationByConstructorExample {
    public static void main(String[] args) {
        System.out.println("main starts");
        new SomeClass();
        System.out.println("main ends");
    }
}

class SomeClass {
    static {
        System.out.println("SomeClass static block");
    }

    public SomeClass() {
        System.out.println("SomeClass constructor");
    }
}
