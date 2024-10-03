package samples.inheritance.instancefield;

public class InheritanceInstanceField {
    public static void main(String[] args) {
        Child child = new Child();
        int sum1 = // sum must be 14
                child.getChildOnlyField() // 5 from Child
                        + child.getParentField() // 3 from Parent (absent in Child, is taken from Parent)
                        + child.getGrandParentField() // 4 from Parent (Parent shadows GrandParent)
                        + child.getGrandParentOnlyField(); // 2 from GrandParent (nor Child neither Parent have it)

        child.setGrandParentOnlyField(102);

        int sum2 = // sum must be 114
                child.getChildOnlyField() // 5 from Child
                        + child.getParentField() // 3 from Parent (absent in Child, is taken from Parent)
                        + child.getGrandParentField() // 4 from Parent (Parent shadows GrandParent)
                        + child.getGrandParentOnlyField(); // 102 from modified GrandParent (nor Child neither Parent have it)

        int result = sum1 + sum2; // sum must be 128
    }
}

class GrandParent {
    protected int grandParentField = 1;
    protected int grandParentOnlyField = 2;
}

class Parent extends GrandParent {
    protected int parentField = 3;
    protected int grandParentField = 4;
}

class Child extends Parent {
    private int childOnlyField = 5;

    public int getChildOnlyField() {
        return childOnlyField;
    }

    public int getParentField() {
        return parentField;
    }

    public int getGrandParentField() {
        return grandParentField;
    }

    public int getGrandParentOnlyField() {
        return grandParentOnlyField;
    }

    public void setGrandParentOnlyField(int grandParentOnlyField) {
        this.grandParentOnlyField = grandParentOnlyField;
    }
}
