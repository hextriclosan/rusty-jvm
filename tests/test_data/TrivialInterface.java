package samples.inheritance.interfaces.trivial;

public class TrivialInterface {
    public static void main(String[] args) {
        Subtraction subtraction = new SubtractionImpl();

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
