package samples.inheritance.interfaces.inheritedimplementation;

public class InheritedImplementationInterface {
    public static void main(String[] args) {
        Subtraction subtraction = new SubtractionImplChild();

        int one = 100;
        int two = 300;
        int result = subtraction.sub(one, two);
    }
}

interface Subtraction {
    int sub(int first, int second);
}

class SubtractionImpl implements Subtraction {
    @Override
    public int sub(int first, int second) {
        return first - second;
    }
}

class SubtractionImplChild extends SubtractionImpl {
}
