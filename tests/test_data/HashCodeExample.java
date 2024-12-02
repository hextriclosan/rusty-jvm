package samples.javacore.hashcodes.trivial;

public class HashCodeExample {
    public static void main(String[] args) {
        CustomClass customObject1 = new CustomClass(10);
        CustomClass customObject2 = new CustomClass(20);
        int customHashCode1 = customObject1.hashCode();
        int customHashCode2 = customObject2.hashCode();
        int identityHashCode1 = System.identityHashCode(customObject1);
        int identityHashCode2 = System.identityHashCode(customObject2);
        int bit0 = customHashCode1 == 310 ? 1 : 0;
        int bit1 = customHashCode2 == 620 ? 1 : 0;
        int bit2 = customHashCode1 != identityHashCode1 ? 1 : 0;
        int bit3 = customHashCode2 != identityHashCode2 ? 1 : 0;

        Object object1 = new Object();
        Object object2 = new Object();
        int objectHashCode1 = object1.hashCode();
        int objectHashCode2 = object2.hashCode();
        int bit4 = objectHashCode1 != objectHashCode2 ? 1 : 0;
        int objectIdentityHashCode1 = System.identityHashCode(object1);
        int objectIdentityHashCode2 = System.identityHashCode(object2);
        int bit5 = objectHashCode1 == objectIdentityHashCode1 ? 1 : 0;
        int bit6 = objectHashCode2 == objectIdentityHashCode2 ? 1 : 0;

        int bit7 = System.identityHashCode(null) == 0 ? 1 : 0;

        int result = 0;
        result = setBit(result, 0, bit0);
        result = setBit(result, 1, bit1);
        result = setBit(result, 2, bit2);
        result = setBit(result, 3, bit3);
        result = setBit(result, 4, bit4);
        result = setBit(result, 5, bit5);
        result = setBit(result, 6, bit6);
        result = setBit(result, 7, bit7);
    }

    private static int setBit(int num, int position, int value) {
        return value == 0 ? num & ~(1 << position) : num | (1 << position);
    }
}

class CustomClass {
    private final int value;

    CustomClass(int value) {
        this.value = value;
    }

    @Override
    public int hashCode() {
        return value * 31;
    }
}
