
public class StaticFieldsUser {
    public static void main(String[] args) {
        int first = 11;
        int second = 10000;
        StaticFields.sub(first, second);
        StaticFields.add(first, second);
        StaticFields.mul(first, second);
        int result = StaticFields.getResultSub() + StaticFields.getResultAdd() + StaticFields.getResultMul();
    }
}
