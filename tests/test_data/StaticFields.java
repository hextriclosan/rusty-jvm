
public class StaticFields {

    private static int resultSub;
    private static int resultAdd;
    private static int resultMul;

    public static int getResultSub() {
        return resultSub;
    }

    public static int getResultAdd() {
        return resultAdd;
    }

    public static int getResultMul() {
        return resultMul;
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
