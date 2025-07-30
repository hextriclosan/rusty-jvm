package samples.reflection.trivial.getinterfacesexample;

import java.util.HashMap;
import java.util.Map;

public class GetInterfacesExample {
    public static void main(String[] args) {
        // Interface itself (does not have interfaces, unless it extends another)
        printInterfaces(Map.class);

        // Interface extending another interface
        printInterfaces(ChildInterface.class);

        printInterfaces(HashMap.class);

        printInterfaces(String[].class);

        printInterfaces(int.class);
    }

    static void printInterfaces(Class<?> clazz) {
        System.out.println("Interfaces implemented by " + clazz.getName() + ":");
        Class<?>[] interfaces = clazz.getInterfaces();
        for (Class<?> iface : interfaces) {
            System.out.println('\t' + iface.getName());
        }
    }

    interface ChildInterface<K, V> extends Map<K, V>, Runnable {
    }
}
