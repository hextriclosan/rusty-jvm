package samples.reflection.trivial.classgetname;

public class ClassGetNameExample {

    public static void main(String[] args) {
        String stringName = String.class.getName();
        String unicodeBlockName = Character.UnicodeBlock.class.getName();
        String byteName = byte.class.getName();
        String objectsArrayName = (new Object[3]).getClass().getName();
        var intArr = new int[3][4][5][6][7][8][9];
        String intArrayName = intArr.getClass().getName();
        String intSubArrayName = intArr[0].getClass().getName();

        System.out.println(stringName);
        System.out.println(unicodeBlockName);
        System.out.println(byteName);
        System.out.println(objectsArrayName);
        System.out.println(intArrayName);
        System.out.println(intSubArrayName);
    }
}
