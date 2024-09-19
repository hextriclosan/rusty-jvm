
public class InstanceFieldsUserLong {
    public static void main(String[] args) {
        long first = 42_949_672_980L/*h=10,l=20*/;
        long second = 128_849_018_920L/*h=30,l=40*/;
        InstanceFieldsLong[] instances = new InstanceFieldsLong[] { new InstanceFieldsLong(), new InstanceFieldsLong(), new InstanceFieldsLong() };
        instances[0].sub(first, second);
        instances[1].add(first, second);
        instances[2].mul(first, second);
        long result = instances[0].getResultSub() + instances[1].getResultAdd() + instances[2].getResultMul();
    }
}
