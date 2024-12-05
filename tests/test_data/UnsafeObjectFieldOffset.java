// javac --add-exports java.base/jdk.internal.misc=ALL-UNNAMED  -d . UnsafeObjectFieldOffset.java

package samples.jdkinternal.unsafe.objectfieldoffset;

import jdk.internal.misc.Unsafe;

public class UnsafeObjectFieldOffset {

    public static void main(String[] args) {
        One one = new One(1, 2);
        boolean oneFieldOneSet = one.compareAndSetFieldOne(1, 10);
        boolean oneFieldThreeSet = one.compareAndSetFieldThree(2, 20);
        int oneFieldOne = one.getFieldOne();
        int oneFieldThree = one.getFieldThree();
        int bit0 = oneFieldOneSet && oneFieldOne == 10 ? 1 : 0;
        int bit1 = oneFieldThreeSet && oneFieldThree == 20 ? 1 : 0;

        Two two = new Two(-1, -2, -3);
        int twoFieldOne = two.getFieldOne(); // -1
        int twoFieldTwo = two.getFieldTwo(); // -2
        int twoFieldThree = two.getFieldThree(); // -3
        int twoFieldThreeFromParent = two.getFieldThreeFromParent(); // -27
        int bit2 = twoFieldOne == -1 && twoFieldTwo == -2 && twoFieldThree == -3 && twoFieldThreeFromParent == -27
            ? 1
            : 0;

        boolean twoFieldOneSet = two.compareAndSetFieldOne(-1, -10);
        boolean twoFieldTwoSet = two.compareAndSetFieldTwo(-2, -20);
        boolean twoFieldThreeSet = two.compareAndSetFieldThree(-3, -30);
        boolean twoFieldThreeFromParentSet = two.compareAndSetFieldThreeFromParent(-27, -270);
        int twoFieldOneAfterSet = two.getFieldOne(); // -10
        int twoFieldTwoAfterSet = two.getFieldTwo(); // -20
        int twoFieldThreeAfterSet = two.getFieldThree(); // -30
        int twoFieldThreeFromParentAfterSet = two.getFieldThreeFromParent(); // -270
        int bit3 = twoFieldOneSet && twoFieldOneAfterSet == -10 ? 1 : 0;
        int bit4 = twoFieldTwoSet && twoFieldTwoAfterSet == -20 ? 1 : 0;
        int bit5 = twoFieldThreeSet && twoFieldThreeAfterSet == -30 ? 1 : 0;
        int bit6 = twoFieldThreeFromParentSet && twoFieldThreeFromParentAfterSet == -270 ? 1 : 0;

        int result = 0;
        result = setBit(result, 0, bit0);
        result = setBit(result, 1, bit1);
        result = setBit(result, 2, bit2);
        result = setBit(result, 3, bit3);
        result = setBit(result, 4, bit4);
        result = setBit(result, 5, bit5);
        result = setBit(result, 6, bit6);
        System.out.println(result);
    }

    private static int setBit(int num, int position, int value) {
        return value == 0 ? num & ~(1 << position) : num | (1 << position);
    }
}

class One {
    protected static final Unsafe U = Unsafe.getUnsafe();
    private static final long FIELD_ONE_OFFSET = U.objectFieldOffset(One.class, "fieldOne");
    protected static final long FIELD_THREE_OFFSET = U.objectFieldOffset(One.class, "fieldThree");

    // Each field should have a particular offset which is the same for all instances of the class
    // no matter if the class is parent of another class or not.
    int placeholder1; // possible offset 0
    int placeholder2; // possible offset 4
    int fieldOne;     // possible offset 8
    int fieldThree;   // possible offset 12
    public One(int fieldOne, int fieldThree) {
        this.fieldOne = fieldOne;
        this.fieldThree = fieldThree;
    }

    public int getFieldOne() {
        return fieldOne;
    }

    public int getFieldThree() {
        return fieldThree;
    }

    protected boolean compareAndSetFieldOne(int expect, int update) {
        return U.compareAndSetInt(this, FIELD_ONE_OFFSET, expect, update);
    }

    protected boolean compareAndSetFieldThree(int expect, int update) {
        return U.compareAndSetInt(this, FIELD_THREE_OFFSET, expect, update);
    }
}

class Two extends One {
    private static final long FIELD_TWO_OFFSET = U.objectFieldOffset(Two.class, "fieldTwo");
    private static final long FIELD_THREE_OFFSET = U.objectFieldOffset(Two.class, "fieldThree");

    int placeholder10; // possible offset 16
    int fieldTwo; // possible offset 20
    int fieldThree; // possible offset 24, shadows One.fieldThree
    public Two(int fieldOne, int fieldTwo, int fieldThree) {
        super(fieldOne, fieldThree * 9);
        this.fieldTwo = fieldTwo;
        this.fieldThree = fieldThree;
    }

    public int getFieldTwo() {
        return fieldTwo;
    }

    @Override
    public int getFieldThree() {
        return fieldThree;
    }

    public int getFieldThreeFromParent() {
        return super.fieldThree;
    }

    protected boolean compareAndSetFieldTwo(int expect, int update) {
        return U.compareAndSetInt(this, FIELD_TWO_OFFSET, expect, update);
    }

    protected boolean compareAndSetFieldThree(int expect, int update) {
        return U.compareAndSetInt(this, FIELD_THREE_OFFSET, expect, update);
    }

    protected boolean compareAndSetFieldThreeFromParent(int expect, int update) {
        return U.compareAndSetInt(this, One.FIELD_THREE_OFFSET, expect, update);
    }
}
