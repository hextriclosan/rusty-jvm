package samples.system.getpropertyexample;

public class SystemGetPropertyExample {
    public static void main(String[] args) {
        generateJson("other.property", "line.separator", "sun.cpu.endian", "os.version", "user.dir", "os.name",
        "file.separator", "path.separator", "java.home", "java.io.tmpdir", "sun.boot.library.path");
    }

    private static void generateJson(String... properties) {
        System.out.print("{");
        for (int i = 0; i < properties.length; i++) {
            String name = properties[i];
            String value = escapeString(System.getProperty(name));
            System.out.print("\"");
            System.out.print(name);
            System.out.print("\": \"");
            System.out.print(value);
            System.out.print("\"");
            if (i < properties.length - 1) {
                System.out.print(", ");
            }
        }
        System.out.println("}");
    }

    private static String escapeString(String input) {
        if (input == null) {
            return null;
        }
        return input.replace("\\", "\\\\")
                    .replace("\"", "\\\"")
                    .replace("\n", "\\n")
                    .replace("\r", "\\r")
                    .replace("\t", "\\t");
    }
}
