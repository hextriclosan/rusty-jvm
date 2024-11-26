package samples.inheritance.usegrandparentmethodviasuper;

public class UseGrandParentMethodViaSuperExample {
    public static void main(String[] args) {
        Child child = new Child();
        int result = child.invoke();
    }
}

class GrandParent {
    int work() {
        return 1337;
    }
}

class Parent extends GrandParent {
}

class Child extends Parent {
    public int invoke() {
        return super.work();
    }
}
