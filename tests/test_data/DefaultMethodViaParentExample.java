package samples.inheritance.defaultmethodviaparent;

public class DefaultMethodViaParentExample {
    public static void main(String[] args) {
        Child child = new Child();

        int result = child.getValue();
        System.out.println(result);
    }
}

class Parent implements Interface {
}

class Child extends Parent {
    @Override
    public int getValue() {
        return super.getValue();
    }
}

interface Interface {
    default int getValue() {
        return 42;
    }
}
