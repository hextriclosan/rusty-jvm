
public class StaticFields {

    private static int resultSub;
    private static int resultAdd;
    private static int resultMul;

    public static void main(String[] args) {
        int first = 11;
        int second = 1000;
        sub(first, second);
        add(first, second);
        mul(first, second);
        int result = resultSub + resultAdd + resultMul;
    }

    public static void sub(int first, int second) {
        resultSub = first - second;
    }

    public static void add(int first, int second) {
        resultAdd = first + second;
    }

    public static void mul(int first, int second) {
        resultMul = first * second;
    }

}
