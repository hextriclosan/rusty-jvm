package samples.inheritance.staticmethod;

public class ParentStaticMethodCalledViaChild {
    public static void main(String[] args) {
        Child.parentMethod();
    }
}

class Parent {
    protected static void parentMethod() {
        System.out.println("Parent method called");
    }
}

class Child extends Parent {
}
