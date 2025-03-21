package samples.inheritance.parentstaticinit;

public class InitParentStaticBlockBeforeChild {
    public static void main(String[] args) {
        Child.childMethod();
    }
}

class Parent {
    static {
        System.out.println("Parent static block");
    }
}

class Child extends Parent {
    static {
        System.out.println("Child static block");
    }

    public static void childMethod() {
        System.out.println("Child method called");
    }
}
