package samples.javacore.cast.arrays;

import static java.lang.String.valueOf;

public class ArrayCastExample {
    public static void main(String[] args) {
        int[] ints = new int[] {1, 2, 3};
        String[] strings = new String[] {"Hello", "World", "!"};
        Object stringsObject = strings;
        System.out.println(valueOf(stringsObject).substring(0, 19));
        Object intsObject = ints;
        System.out.println(valueOf(intsObject).substring(0, 2));

        Object[] objects = (Object[]) stringsObject;
        for (Object o : objects) {
            System.out.print(o);
        }
        System.out.println();
    }
}
