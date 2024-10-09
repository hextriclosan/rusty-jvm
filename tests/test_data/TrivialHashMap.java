package samples.javabase.util.hashmap.trivial;

import java.util.Map;
import java.util.HashMap;

public class TrivialHashMap {

    public static void main(String[] args) {
        Map<Integer, Integer> map = new HashMap<>();
        map.put(13, -999);

        int result = map.get(13);
    }
}
