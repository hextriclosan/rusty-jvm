
public class InstanceFieldsLong {

    private long resultSub;
    private long resultAdd;
    private long resultMul;

    public void sub(long first, long second) {
        resultSub = first - second;
    }

    public void add(long first, long second) {
        resultAdd = first + second;
    }

    public void mul(long first, long second) {
        resultMul = first * second;
    }

    public long getResultSub() {
        return resultSub;
    }

    public long getResultAdd() {
        return resultAdd;
    }

    public long getResultMul() {
        return resultMul;
    }
}
