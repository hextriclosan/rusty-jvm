package samples.javacore.hashcodes.trivial;

public class HashCodeExample {
    public static void main(String[] args) {
        CustomClass customObject1 = new CustomClass(10);
        CustomClass customObject2 = new CustomClass(20);
        int customHashCode1 = customObject1.hashCode();
        int customHashCode2 = customObject2.hashCode();
        int identityHashCode1 = System.identityHashCode(customObject1);
        int identityHashCode2 = System.identityHashCode(customObject2);
        System.out.println("customHashCode1: " + customHashCode1); // 310
        System.out.println("customHashCode2: " + customHashCode2); // 620
        System.out.println("customHashCode1 != identityHashCode1: " + (customHashCode1 != identityHashCode1)); // true
        System.out.println("customHashCode2 != identityHashCode2: " + (customHashCode2 != identityHashCode2)); // true

        Object object1 = new Object();
        Object object2 = new Object();
        int objectHashCode1 = object1.hashCode();
        int objectHashCode2 = object2.hashCode();
        System.out.println("objectHashCode1 != objectHashCode2: " + (objectHashCode1 != objectHashCode2)); // true

        int objectIdentityHashCode1 = System.identityHashCode(object1);
        int objectIdentityHashCode2 = System.identityHashCode(object2);
        System.out.println("objectHashCode1 == objectIdentityHashCode1: " + (objectHashCode1 == objectIdentityHashCode1)); // true
        System.out.println("objectHashCode2 == objectIdentityHashCode2: " + (objectHashCode2 == objectIdentityHashCode2)); // true

        int hashCodeOfNull = System.identityHashCode(null); //According to docs: The hash code for the null reference is zero.
        System.out.println("hashCodeOfNull: " + hashCodeOfNull);
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
