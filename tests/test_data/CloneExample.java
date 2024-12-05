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

        int[] intArray = new int[]{1, 2, 3};
        int[] anotherIntArray = intArray.clone();
        int bit3 = intArray != anotherIntArray ? 1 : 0;
        int bit4 = intArray[0] == anotherIntArray[0] && intArray[1] == anotherIntArray[1] && intArray[2] == anotherIntArray[2] ? 1 : 0;
        anotherIntArray[1] = 200;
        int bit5 = intArray[1] == 2 ? 1 : 0;

        Object first = new Object(), second = new Object(), third = new Object();
        Object[] objArray = new Object[]{first, second, third};
        Object[] anotherObjArray = objArray.clone();
        int bit6 = objArray != anotherObjArray ? 1 : 0;
        int bit7 = objArray[0] == anotherObjArray[0] && objArray[1] == anotherObjArray[1] && objArray[2] == anotherObjArray[2] ? 1 : 0;
        anotherObjArray[1] = new Object();
        int bit8 = objArray[1] == second ? 1 : 0;

//         int[][] intMatrix = new int[][]{{1, 2}, {3, 4}};
//         int[][] anotherIntMatrix = intMatrix.clone();
//         int bit9 = intMatrix != anotherIntMatrix ? 1 : 0;
//         int bit10 = intMatrix[0] == anotherIntMatrix[0] && intMatrix[1] == anotherIntMatrix[1] ? 1 : 0;
//         anotherIntMatrix[1][1] = 40;
//         int bit11 = intMatrix[1][1] == 40 ? 1 : 0;

        int result = 0;
        result = setBit(result, 0, bit0);
        result = setBit(result, 1, bit1);
        result = setBit(result, 2, bit2);
        result = setBit(result, 3, bit3);
        result = setBit(result, 4, bit4);
        result = setBit(result, 5, bit5);
        result = setBit(result, 6, bit6);
        result = setBit(result, 7, bit7);
        result = setBit(result, 8, bit8);
//         result = setBit(result, 9, bit9);
//         result = setBit(result, 10, bit10);
//         result = setBit(result, 11, bit11);
        System.out.println(result);
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
