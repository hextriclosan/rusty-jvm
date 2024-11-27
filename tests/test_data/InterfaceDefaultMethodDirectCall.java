package samples.inheritance.interfacedefaultmethoddirectcall;

public class InterfaceDefaultMethodDirectCall {
    public static void main(String[] args) {
        SomeClass someClass = new SomeClass();
        int valueDirectCall = someClass.getValueDirectCall();
        int valueOverriddenInImpl = someClass.getValueOverriddenInImpl();
        int valueInBaseClass = someClass.getValueInBaseClass();
        int valueFromExtendedInterface = someClass.getValueFromExtendedInterface();
        int result = valueDirectCall | valueOverriddenInImpl | valueInBaseClass | valueFromExtendedInterface;
    }
}

interface SomeInterface {
    default int getValueOverriddenInImpl() {
        return 16;
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
    default int getValueFromExtendedInterface() {
        return 8;
    }
}

interface AnotherInterface extends AnotherInterfaceBase {
}

class BaseClass implements AnotherInterface {
    public int getValueInBaseClass() {
        return 4;
    }
}

class SomeClass extends BaseClass implements SomeInterface {
    @Override
    public int getValueOverriddenInImpl() {
        return 2;
    }
    @Override
    public int nonDefaultMethod() {
        return 64;
    }
}
