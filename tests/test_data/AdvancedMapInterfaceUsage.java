package samples.javabase.util.mapinterface.usage;

import java.util.HashMap;
import java.util.TreeMap;
import java.util.Map;
import java.util.function.BiFunction;

public class AdvancedMapInterfaceUsage {
    public static void main(String[] args) {
        testMap(new HashMap<>());
        testMap(new TreeMap<>());
    }

    public static void testMap(Map<String, String> map) {
        System.out.println(map.getClass().getName());
        map.put("one", "1");
        map.put("two", "2");
        map.put("three", "3");
        if (map.size() == 3) {
            System.out.println("Map size is 3");
        }

        if (map.containsKey("two")) {
            System.out.println("Map contains key 'two'");
            map.put("two", "22");
        }

        if (map.remove("three", "3")) {
            System.out.println("Removed key 'three' with value '3'");
        }

        String value = map.getOrDefault("four", "4");
        if (value.equals("4")) {
            System.out.println("Value for key 'four' is '4'");
        }

        int sum = 0;
        for (Map.Entry<String, String> entry : map.entrySet()) {
            sum += entry.getKey().length() + entry.getValue().length(); // Adding lengths of keys and values
        }
        if (sum == 9) {
            System.out.println("Sum of lengths of keys and values is 9");
        }

        // Advanced Cases
        map.put("nullKey", null);
        if (map.containsKey("nullKey") && map.get("nullKey") == null) {
            System.out.println("Map contains key 'nullKey' with null value");
        }

        map.merge("one", "10", new BiFunction<String, String, String>() {
            @Override
            public String apply(String oldValue, String newValue) {
                return oldValue.concat(newValue); // Concatenate old and new values
            }
        });
        if ("110".equals(map.get("one"))) {
            System.out.println("Merged key 'one' with value '10'");
        }

        map.compute("two", new BiFunction<String, String, String>() {
            @Override
            public String apply(String key, String oldValue) {
                return oldValue == null ? "0" : oldValue.concat("2"); // Append "2" if value exists, or return "0"
            }
        });
        if ("222".equals(map.get("two"))) {
            System.out.println("Computed key 'two' with value '2'");
        }

        map.replaceAll(new BiFunction<String, String, String>() {
            @Override
            public String apply(String key, String oldValue) {
                return oldValue != null ? oldValue.toUpperCase() : "DEFAULT";
            }
        });
        if ("DEFAULT".equals(map.get("nullKey"))) {
            System.out.println("Replaced null value with 'DEFAULT'");
        }

        int count = 0;
        for (String key : map.keySet()) {
            if (key != null && key.startsWith("n")) {
                count++;
            }
        }
        if (count == 1) {
            System.out.println("One key starts with 'n'");
        }

        map.clear();
        if (map.isEmpty()) {
            System.out.println("Map is empty");
        }

        System.out.println();
    }
}
