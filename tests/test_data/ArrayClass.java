package samples.reflection.trivial;

public class ArrayClass {
    public static void main(String[] args) {
        var classArray3d = int[][][].class;
        int bit0 = classArray3d.isPrimitive() ? 0 : 1;
        int bit1 = classArray3d.isArray() ? 1 : 0;

        Class<?> classArray2d = classArray3d.getComponentType();
        int bit2 = classArray2d.isPrimitive() ? 0 : 1;
        int bit3 = classArray2d.isArray() ? 1 : 0;
        int bit4 = classArray2d == int[][].class ? 1 : 0;

        Class<?> classArray1d = classArray2d.getComponentType();
        int bit5 = classArray1d.isPrimitive() ? 0 : 1;
        int bit6 = classArray1d.isArray() ? 1 : 0;
        int bit7 = classArray1d == int[].class ? 1 : 0;

        Class<?> classInt = classArray1d.getComponentType();
        int bit8 = classInt.isPrimitive() ? 1 : 0;
        int bit9 = classInt.isArray() ? 0 : 1;
        int bit10 = classInt == int.class ? 1 : 0;
        int bit11 = classInt.getComponentType() == null ? 1 : 0;

        var classStringArray = String[].class;
        int bit12 = classStringArray.isPrimitive() ? 0 : 1;
        int bit13 = classStringArray.isArray() ? 1 : 0;
        Class<?> classString = classStringArray.getComponentType();
        int bit14 = classString.isPrimitive() ? 0 : 1;
        int bit15 = classString.isArray() ? 0 : 1;
        int bit16 = classString == String.class ? 1 : 0;
        int bit17 = classString.getComponentType() == null ? 1 : 0;

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
        result = setBit(result, 9, bit9);
        result = setBit(result, 10, bit10);
        result = setBit(result, 11, bit11);
        result = setBit(result, 12, bit12);
        result = setBit(result, 13, bit13);
        result = setBit(result, 14, bit14);
        result = setBit(result, 15, bit15);
        result = setBit(result, 16, bit16);
        result = setBit(result, 17, bit17);
        System.out.println(result);
    }

    private static int setBit(int num, int position, int value) {
        return value == 0 ? num & ~(1 << position) : num | (1 << position);
    }
}
