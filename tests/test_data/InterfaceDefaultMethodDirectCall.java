package samples.inheritance.interfacedefaultmethoddirectcall;

public class InterfaceDefaultMethodDirectCall {
    public static void main(String[] args) {
        SomeClass someClass = new SomeClass();
        int valueDirectCall = someClass.getValueDirectCall();
        int valueOverriddenInImpl = someClass.getValueOverriddenInImpl();
        int valueInBaseClass = someClass.getValueInBaseClass();
        int valueFromExtendedInterface = someClass.getValueFromExtendedInterface();
        int valueDefaultPlusFromImpl = someClass.getValueDefaultPlusFromImpl();
        int result = valueDirectCall | valueOverriddenInImpl | valueInBaseClass | valueFromExtendedInterface
                | valueDefaultPlusFromImpl;
        System.out.println(result);
    }
}

interface SomeInterface {
    default int getValueOverriddenInImpl() {
        return -100;
    }

    default int getValueDirectCall() {
        return 1;
    }

    default int getValueInBaseClass() {
        return 32;
    }

    int nonDefaultMethod();
}

interface AnotherInterfaceBase {
    int DEFAULT_VALUE = -1;

    default int getValueFromExtendedInterface() {
        return 8;
    }

    default int getValueDefaultPlusFromImpl() {
        return DEFAULT_VALUE + getValueNonDefaultMethod();
    }

    int getValueNonDefaultMethod();
}

interface AnotherInterface extends AnotherInterfaceBase {
}

class BaseClass implements AnotherInterface {
    public int getValueInBaseClass() {
        return 4;
    }

    public int getValueNonDefaultMethod() {
        return 17;
    }
}

class SomeClass extends BaseClass implements SomeInterface {
    @Override
    public int getValueOverriddenInImpl() {
        return 2;
    }

    @Override
    public int nonDefaultMethod() {
        return -1000;
    }
}

