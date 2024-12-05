package samples.javabase.util.properties.trivial;

import java.util.Map;
import java.util.HashMap;
import java.util.Properties;

public class PropertiesTrivial {
    public static void main(String[] args) {
        Map<String, String> initialProps = new HashMap<>();
            initialProps.put("private.property1", "10");
            initialProps.put("public.property1", "20");
            initialProps.put("private.property2", "30");
            initialProps.put("public.property2", "40");

        Properties properties = createProperties(initialProps);
        int sum = 0;
        sum += Integer.parseInt(properties.getProperty("public.property1"));
        sum += Integer.parseInt(properties.getProperty("public.property2"));

        int result = sum;
        System.out.println(result);
    }

    private static Properties createProperties(Map<String, String> initialProps) {
        Properties properties = new Properties(initialProps.size());
        for (var entry : initialProps.entrySet()) {
            String prop = entry.getKey();
            switch (prop) {
                case "private.property1":
                case "private.property2":
                    break;
                default:
                    properties.put(prop, entry.getValue());
            }
        }
        return properties;
    }
}
