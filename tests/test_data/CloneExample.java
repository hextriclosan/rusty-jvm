package samples.javacore.cloneable.trivial;

import java.util.Arrays;
import java.util.Objects;

public class CloneExample {
    public static void main(String[] args) {
        CloneableImpl cloneable = new CloneableImpl(42, "first");
        CloneableImpl anotherCloneable = cloneable.clone();
        if (cloneable != anotherCloneable && cloneable.equals(anotherCloneable)) {
            System.out.println("cloneable and anotherCloneable have different references but the same content");
        }

        anotherCloneable.intField = 1337;
        anotherCloneable.stringField = "second";
        if (cloneable.intField == 42 && cloneable.stringField.equals("first")) {
            System.out.println("cloneable is not affected by changes in anotherCloneable");
        }

        int[] intArray = new int[]{1, 2, 3};
        int[] anotherIntArray = intArray.clone();
        if (intArray != anotherIntArray && Arrays.equals(intArray, anotherIntArray)) {
            System.out.println("intArray and anotherIntArray have different references but the same content");
        }
        anotherIntArray[1] = 200;
        if (intArray[1] == 2) {
            System.out.println("intArray is not affected by changes in anotherIntArray");
        }

        Object first = new Object(), second = new Object(), third = new Object();
        Object[] objArray = new Object[]{first, second, third};
        Object[] anotherObjArray = objArray.clone();
        if (objArray != anotherObjArray && Arrays.equals(objArray, anotherObjArray)) {
            System.out.println("objArray and anotherObjArray have different references but the same content");
        }
        anotherObjArray[1] = new Object();
        if (objArray[1] == second) {
            System.out.println("objArray is not affected by changes in anotherObjArray");
        }

        int[][] intMatrix = new int[][]{{1, 2}, {3, 4}};
        int[][] anotherIntMatrix = intMatrix.clone();
        if (intMatrix != anotherIntMatrix && Arrays.equals(intMatrix, anotherIntMatrix)) {
            System.out.println("intMatrix and anotherIntMatrix have different references but the same content");
        }
        anotherIntMatrix[1][1] = 40;
        if(intMatrix[1][1] == 40) {
            System.out.println("intMatrix is affected by changes in anotherIntMatrix");
        }
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

    @Override
    public boolean equals(Object o) {
        if (o == null || getClass() != o.getClass()) return false;
        CloneableImpl cloneable = (CloneableImpl) o;
        return intField == cloneable.intField && Objects.equals(stringField, cloneable.stringField);
    }

    @Override
    public int hashCode() {
        return Objects.hash(intField, stringField);
    }
}
