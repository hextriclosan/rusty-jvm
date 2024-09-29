package samples.inheritance.staticfield;

public class InheritanceStaticField {
    public static void main(String[] args) {
        int sum1 = // sum must be 14
                Child.childOnlyField // 5 from Child
                        + Child.parentField // 3 from Parent (absent in Child, is taken from Parent)
                        + Child.grandParentField // 4 from Parent (Parent shadows GrandParent)
                        + Child.grandParentOnlyField; // 2 from GrandParent (nor Child neither Parent have it)

        Child.grandParentOnlyField = 102;

        int sum2 = // sum must be 114
                Child.childOnlyField // 5 from Child
                        + Child.parentField // 3 from Parent (absent in Child, is taken from Parent)
                        + Child.grandParentField // 4 from Parent (Parent shadows GrandParent)
                        + Child.grandParentOnlyField; // 102 from modified GrandParent (nor Child neither Parent have it)

        int result = sum1 + sum2; // sum must be 128
    }
}

class GrandParent {
    public static int grandParentField = 1;
    public static int grandParentOnlyField = 2;
}

class Parent extends GrandParent {
    public static int parentField = 3;
    public static int grandParentField = 4;
}

class Child extends Parent {
    public static int childOnlyField = 5;
}
