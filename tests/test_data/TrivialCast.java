package samples.javacore.cast.trivial;

import java.util.AbstractMap;
import java.util.HashMap;

public class TrivialCast {
    public static void main(String[] args) {
        Object o = new HashMap<Integer, Integer>();
        var abstractMap = (AbstractMap<Integer, Integer>) o;

        int result = 1337;
    }
}
