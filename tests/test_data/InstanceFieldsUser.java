
public class InstanceFieldsUser {
    public static void main(String[] args) {
        int first = 11;
        int second = 10000;
        InstanceFields[] instances = new InstanceFields[] { new InstanceFields(), new InstanceFields(), new InstanceFields() };
        instances[0].sub(first, second);
        instances[1].add(first, second);
        instances[2].mul(first, second);
        int result = instances[0].getResultSub() + instances[1].getResultAdd() + instances[2].getResultMul();
    }
}
