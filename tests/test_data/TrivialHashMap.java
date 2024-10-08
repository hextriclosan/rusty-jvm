package samples.javabase.util.hashmap.trivial;

import java.util.Map;
import java.util.HashMap;

public class TrivialHashMap {
    public static void main(String[] args) {
        int result = getSum();
    }

    private static int getSum() {
        Map<Integer, Integer> map = new HashMap<>();
        map.put(1, 10);
        map.put(2, 20);
        map.put(3, 30);

        map.remove(2);
        map.put(1, 50);

        int sum = 0;
        for (var entry : map.entrySet()) {
            sum += entry.getKey() + entry.getValue();
        }
        return sum;
    }
}

