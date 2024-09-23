package samples.fields.instance.longs;

public class InstanceFieldsUserLong {
    public static void main(String[] args) {
        long first = 42_949_672_980L/*h=10,l=20*/;
        long second = 128_849_018_920L/*h=30,l=40*/;
        InstanceFields[] instances = new InstanceFields[] { new InstanceFields(), new InstanceFields(), new InstanceFields() };
        instances[0].sub(first, second);
        instances[1].add(first, second);
        instances[2].mul(first, second);
        long result = instances[0].getResultSub() + instances[1].getResultAdd() + instances[2].getResultMul();
    }
}

class InstanceFields {

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