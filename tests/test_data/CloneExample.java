package samples.javacore.cloneable.trivial;

public class CloneExample {
    public static void main(String[] args) {
        CloneableImpl cloneable = new CloneableImpl(42, "first");
        CloneableImpl anotherCloneable = cloneable.clone();
        int bit0 = cloneable != anotherCloneable ? 1 : 0;
        int bit1 = cloneable.intField == anotherCloneable.intField && cloneable.stringField.equals(anotherCloneable.stringField) ? 1 : 0;
        anotherCloneable.intField = 1337;
        anotherCloneable.stringField = "second";
        int bit2 = cloneable.intField == 42 && cloneable.stringField.equals("first") ? 1 : 0;

        int result = 0;
        result = setBit(result, 0, bit0);
        result = setBit(result, 1, bit1);
        result = setBit(result, 2, bit2);
    }

    private static int setBit(int num, int position, int value) {
        return value == 0 ? num & ~(1 << position) : num | (1 << position);
    }
}

class CloneableImpl implements Cloneable {
    int intField;
    String stringField;

    public CloneableImpl(int intField, String stringField) {
        this.intField = intField;
        this.stringField = stringField;
    }

    @Override
    public CloneableImpl clone() {
        try {
            return (CloneableImpl) super.clone();
        } catch (CloneNotSupportedException e) {
            throw new RuntimeException("Cloning not supported", e);
        }
    }
}
