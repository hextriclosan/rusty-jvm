package samples.fields.instance.ints;

public class InstanceFieldsUserInts {
    public static void main(String[] args) {
        int first = 11;
        int second = 10000;
        InstanceFields one = null;
        one = new InstanceFields();
        InstanceFields[] instances = new InstanceFields[] { one, new InstanceFields(), new InstanceFields() };
        instances[0].sub(first, second);
        instances[1].add(first, second);
        instances[2].mul(first, second);

        int result = (one != instances[1] ? instances[0].getResultSub() : 0) + instances[1].getResultAdd() + instances[2].getResultMul();
        System.out.println(result);
    }
}

class InstanceFields {

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
