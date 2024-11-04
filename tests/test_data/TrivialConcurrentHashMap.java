package samples.javabase.util.concurrent.hashmap.trivial;

import java.util.concurrent.ConcurrentHashMap;

public class TrivialConcurrentHashMap {
    public static void main(String[] args) {
        int result = calc();
    }

    private static int calc() {
        var map = new ConcurrentHashMap<Integer, Integer>();
        map.put(42, 97);

        return map.get(42);
    }
}
