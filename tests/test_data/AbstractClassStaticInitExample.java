package samples.javacore.abstractclasses.abstractclassstaticinit;

public class AbstractClassStaticInitExample {

    public static void main(String[] args) {
        System.out.println(Abstract.getData().getValue());
    }
}

abstract class Abstract {
    static {
        System.out.println("Abstract static init");
        Child.DATA = new Data(42);
    }

    public static Data getData() {
        return Child.DATA;
    }
}

class Child extends Abstract {
    static Data DATA;
}

class Data {
    public Data(int value) {
        System.out.println("Data constructor");
        this.value = value;
    }

    public int getValue() {
        System.out.println("Data getValue");
        return value;
    }

    private final int value;
}
