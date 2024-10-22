package samples.javacore.cast.trivial;

import java.util.AbstractMap;
import java.util.HashMap;

@SuppressWarnings("unchecked")
public class TrivialCast {
    public static void main(String[] args) {
        Object o1 = new HashMap<Integer, Integer>();
        var abstractMap = (AbstractMap<Integer, Integer>) o1;

        Object o2 = new long[] { 1, 2, 3 };
        var longArray = (long[]) o2;

        Object o3 = new String[10];
        var stringArray = (String[]) o3;

        int result = 1337;
    }
}
