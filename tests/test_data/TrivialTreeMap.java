package samples.javabase.util.treemap.trivial;

import java.util.Map;
import java.util.TreeMap;

public class TrivialTreeMap {
    public static void main(String[] args) {
        int result = getSum();
    }

    private static int getSum() {
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
}
