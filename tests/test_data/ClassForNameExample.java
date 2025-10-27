package samples.reflection.trivial.forname;

public class ClassForNameExample {

    public static void main(String[] args) {
        loadAndPrint("samples.reflection.trivial.forname.First", true);
        loadAndPrint("samples.reflection.trivial.forname.Second", false);
        loadAndPrint("samples.reflection.trivial.forname.NonExisting", true);
    }

    private static void loadAndPrint(String className, boolean initialize) {
        try {
            Class<?> clazz = Class.forName(className, initialize, ClassForNameExample.class.getClassLoader());
            System.out.printf("Loaded class: %s (initialized=%s)%n", clazz.getName(), initialize);
        } catch (ClassNotFoundException e) {
            System.out.printf("ClassNotFoundException: %s - %s%n", className, e);
        }
    }
}

class First {
    static {
        System.out.println(">>> First static block executed");
    }
}

class Second {
    static {
        System.out.println(">>> Second static block executed");
    }
}
