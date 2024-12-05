package samples.javabase.util.treemap.trivial;

import java.util.Map;
import java.util.TreeMap;

public class TrivialTreeMap {
    public static void main(String[] args) {
        int sumOfIntegerMap = getSumOfIntegerMap();
        String sorted = getSortedFromStringMap();
        int result = sumOfIntegerMap == 84 && "150330".equals(sorted) ? 1 : 0;
        System.out.println(result);
    }

    private static int getSumOfIntegerMap() {
        Map<Integer, Integer> map = new TreeMap<>();
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

    private static String getSortedFromStringMap() {
        Map<String, String> map = new TreeMap<>();
        map.put("1", "10");
        map.put("2", "20");
        map.put("3", "30");

        map.remove("2");
        map.put("1", "50");

        StringBuilder sb = new StringBuilder();
        for (var entry : map.entrySet()) {
            sb.append(entry.getKey()).append(entry.getValue());
        }
        return sb.toString();
    }
}
