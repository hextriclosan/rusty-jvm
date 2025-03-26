// javac -XDstringConcat=inline -d . InitializedChildFieldInParentBlockAccessedByChild.java
package samples.staticinit.initialized_child_field_in_parent_block_accessed_by_child;

public class InitializedChildFieldInParentBlockAccessedByChild {
    public static void main(String[] args) {
        System.out.println("entering main");
        System.out.println(Child.childData());
        System.out.println("exiting main");
    }
}

class Parent {
    static {
        System.out.println("Parent static block");
        Child.DATA += "-parent";
    }
}

class Child extends Parent {
    static String DATA = "initial";

    static {
        System.out.println("Child static block");
        DATA += "-child";
    }

    public static String childData() {
        return DATA;
    }
}
