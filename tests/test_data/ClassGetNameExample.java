package samples.reflection.trivial.classgetname;

public class ClassGetNameExample {

    public static void main(String[] args) {
        String stringName = String.class.getName();
        String unicodeBlockName = Character.UnicodeBlock.class.getName();
        String byteName = byte.class.getName();
        String objectsArrayName = (new Object[3]).getClass().getName();
        //String intArrayName = (new int[3][4][5][6][7][8][9]).getClass().getName(); //todo: https://github.com/hextriclosan/rusty-jvm/issues/125

        int result = "java.lang.String".equals(stringName) &&
                "java.lang.Character$UnicodeBlock".equals(unicodeBlockName) &&
                "byte".equals(byteName) &&
                "[Ljava.lang.Object;".equals(objectsArrayName) /*&&
                "[[[[[[[I".equals(intArrayName)*/
                ? 1
                : 0;
        System.out.println(result);
    }
}
