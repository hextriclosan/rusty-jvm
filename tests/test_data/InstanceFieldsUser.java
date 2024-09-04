
public class InstanceFieldsUser {
    public static void main(String[] args) {
        int first = 11;
        int second = 10000;
        InstanceFields instance = new InstanceFields();
        instance.sub(first, second);
        instance.add(first, second);
        instance.mul(first, second);
        int result = instance.getResultSub() + instance.getResultAdd() + instance.getResultMul();
    }
}
