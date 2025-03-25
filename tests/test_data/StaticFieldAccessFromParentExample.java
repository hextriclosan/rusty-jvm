// javac -XDstringConcat=inline -d . StaticFieldAccessFromParentExample.java
package samples.staticinit.parentstaticfieldaccess;

public class StaticFieldAccessFromParentExample {
    public static void main(String[] args) {
        System.out.println("PARENT_FIELD: " + Child.PARENT_FIELD);
        System.out.println("CHILD_FIELD: " + Child.CHILD_FIELD);
        System.out.println("PARENT_INTERFACE_FIELD: " + Child.PARENT_INTERFACE_FIELD);
        System.out.println("CHILD_INTERFACE_FIELD: " + Child.CHILD_INTERFACE_FIELD);
    }
}

class Parent implements ChildInterface {
    static int PARENT_FIELD = 1;
}

class Child extends Parent {
    static int CHILD_FIELD = 2;
}

interface ParentInterface {
    int PARENT_INTERFACE_FIELD = 3;
}

interface ChildInterface extends ParentInterface {
    int CHILD_INTERFACE_FIELD = 4;
}
