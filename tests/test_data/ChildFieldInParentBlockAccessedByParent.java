// javac -XDstringConcat=inline -d . ChildFieldInParentBlockAccessedByParent.java
package samples.staticinit.child_field_in_parent_block_accessed_by_parent;

public class ChildFieldInParentBlockAccessedByParent {
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
    static String DATA;

    static {
        System.out.println("Child static block");
        DATA += "-child";
    }
}
