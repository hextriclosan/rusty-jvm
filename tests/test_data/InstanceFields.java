
public class InstanceFields {

    private int resultSub;
    private int resultAdd;
    private int resultMul;

    public void sub(int first, int second) {
        resultSub = first - second;
    }

    public void add(int first, int second) {
        resultAdd = first + second;
    }

    public void mul(int first, int second) {
        resultMul = first * second;
    }

    public int getResultSub() {
        return resultSub;
    }

    public int getResultAdd() {
        return resultAdd;
    }

    public int getResultMul() {
        return resultMul;
    }
}
