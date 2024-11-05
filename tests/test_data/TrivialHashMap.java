package samples.javabase.util.hashmap.trivial;

import java.util.Map;
import java.util.HashMap;

public class TrivialHashMap {
    public static void main(String[] args) {
        int sumOfIntegerMap = getSumOfIntegerMap();
        int sumOfStringMap = getSumOfStringMap();
        int result = sumOfIntegerMap == 84 && sumOfStringMap == 84 ? 1 : 0;
    }

    private static int getSumOfIntegerMap() {
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

    private static int getSumOfStringMap() {
        Map<String, String> map = new HashMap<>();
        map.put("1", "10");
        map.put("2", "20");
        map.put("3", "30");

        map.remove("2");
        map.put("1", "50");

        int sum = 0;
        for (var entry : map.entrySet()) {
            sum += Integer.parseInt(entry.getKey()) + Integer.parseInt(entry.getValue());
        }
        return sum;
    }
}
