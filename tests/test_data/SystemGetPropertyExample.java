package samples.system.getpropertyexample;

public class SystemGetPropertyExample {
    public static void main(String[] args) {
        print("line.separator");
        print("sun.cpu.endian");
    }

    private static void print(String name) {
        String value = System.getProperty(name);
        System.out.print(name);
        System.out.print(": ");
        System.out.println(value);
    }
}
