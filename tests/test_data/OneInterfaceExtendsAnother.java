package samples.inheritance.interfaces.oneinterfaceextendsanother;

public class OneInterfaceExtendsAnother {
    public static void main(String[] args) {
        SubtractionLongs subtraction = new SubtractionImpl();

        int one = 100;
        int two = 300;
        long result = subtraction.sub(one, two) + subtraction.subLong(one, two);
        System.out.println(result);
    }
}

interface Subtraction {
    int sub(int first, int second);
}

interface SubtractionLongs extends Subtraction {
    long subLong(long first, long second);
}

class SubtractionImpl implements SubtractionLongs {
    @Override
    public int sub(int first, int second) {
        return first - second;
    }

    @Override
    public long subLong(long first, long second) {
        return first - second;
    }
}
