package samples.javabase.util.mapinterface.usage;

import java.util.HashMap;
import java.util.TreeMap;
import java.util.Map;
import java.util.function.BiFunction;

public class AdvancedMapInterfaceUsage {
    public static void main(String[] args) {
        int expected = 2047;
        int hashMapResult = testMap(new HashMap<>());
        int treeMapResult = testMap(new TreeMap<>());
        int result = hashMapResult == expected && treeMapResult == expected ? 1 : 0;
        System.out.println(result);
    }

    public static int testMap(Map<String, String> map) {
        int result = 0;

        map.put("one", "1");
        map.put("two", "2");
        map.put("three", "3");
        int bit0 = map.size() == 3 ? 1 : 0;

        int bit1 = 0;
        if (map.containsKey("two")) {
            map.put("two", "22");
            bit1 = 1;
        }

        int bit2 = map.remove("three", "3") ? 1 : 0;

        String value = map.getOrDefault("four", "4");
        int bit3 = value.equals("4") ? 1 : 0;

        int sum = 0;
        for (Map.Entry<String, String> entry : map.entrySet()) {
            sum += entry.getKey().length() + entry.getValue().length(); // Adding lengths of keys and values
        }
        int bit4 = sum == 9 ? 1 : 0;

        // Advanced Cases
        map.put("nullKey", null);
        int bit5 = map.containsKey("nullKey") && map.get("nullKey") == null ? 1 : 0;

        map.merge("one", "10", new BiFunction<String, String, String>() {
            @Override
            public String apply(String oldValue, String newValue) {
                return oldValue.concat(newValue); // Concatenate old and new values
            }
        });
        int bit6 = "110".equals(map.get("one")) ? 1 : 0;

        map.compute("two", new BiFunction<String, String, String>() {
            @Override
            public String apply(String key, String oldValue) {
                return oldValue == null ? "0" : oldValue.concat("2"); // Append "2" if value exists, or return "0"
            }
        });
        int bit7 = "222".equals(map.get("two")) ? 1 : 0;

        map.replaceAll(new BiFunction<String, String, String>() {
            @Override
            public String apply(String key, String oldValue) {
                return oldValue != null ? oldValue.toUpperCase() : "DEFAULT";
            }
        });
        int bit8 = "DEFAULT".equals(map.get("nullKey")) ? 1 : 0;

        int count = 0;
        for (String key : map.keySet()) {
            if (key != null && key.startsWith("n")) {
                count++;
            }
        }
        int bit9 = count == 1 ? 1 : 0;

        map.clear();
        int bit10 = map.isEmpty() ? 1 : 0;

        result = setBit(result, 0, bit0);
        result = setBit(result, 1, bit1);
        result = setBit(result, 2, bit2);
        result = setBit(result, 3, bit3);
        result = setBit(result, 4, bit4);
        result = setBit(result, 5, bit5);
        result = setBit(result, 6, bit6);
        result = setBit(result, 7, bit7);
        result = setBit(result, 8, bit8);
        result = setBit(result, 9, bit9);
        result = setBit(result, 10, bit10);

        return result;
    }

    private static int setBit(int num, int position, int value) {
        return value == 0 ? num & ~(1 << position) : num | (1 << position);
    }
}
