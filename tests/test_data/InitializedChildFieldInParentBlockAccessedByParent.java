// javac -XDstringConcat=inline -d . InitializedChildFieldInParentBlockAccessedByParent.javac
package samples.staticinit.initialized_child_field_in_parent_block_accessed_by_parent;

public class InitializedChildFieldInParentBlockAccessedByParent {
    public static void main(String[] args) {
        System.out.println("entering main");
        System.out.println(Parent.childData());
        System.out.println("exiting main");
    }
}

class Parent {
    static {
        System.out.println("Parent static block");
        Child.DATA += "-parent";
    }

    public static String childData() {
        return Child.DATA;
    }
}

class Child extends Parent {
    static String DATA = "initial";

    static {
        System.out.println("Child static block");
        DATA += "-child";
    }
}
