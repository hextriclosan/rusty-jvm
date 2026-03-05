package sample.loader.appclassloaderdemo;

import java.lang.reflect.InvocationTargetException;
import java.security.CodeSource;

public class AppClassLoaderDemo {
    public static void main(String[] args) throws ClassNotFoundException, NoSuchMethodException, InvocationTargetException, InstantiationException, IllegalAccessException {
        // Get the system (application) class loader
        ClassLoader appClassLoader = ClassLoader.getSystemClassLoader();
        System.out.println("Application ClassLoader: " + appClassLoader.getName());

        // The parent of the application class loader is usually the platform class loader
        ClassLoader platformClassLoader = appClassLoader.getParent();
        System.out.println("Parent of Application ClassLoader: " + platformClassLoader.getName());

        // The parent of the platform class loader is the bootstrap class loader (represented as null)
        System.out.println("Parent of Platform ClassLoader (Bootstrap): " + platformClassLoader.getParent());

        // Example: Loading a user-defined class with the application class loader

        Class<?> loadMe1 = Class.forName("sample.loader.appclassloaderdemo.LoadMe1", true, appClassLoader);
        ClassLoader loader1 = loadMe1.getClassLoader();
        String loader1Name = (loader1 == appClassLoader) ? loader1.getName() : "NOT_APP_CLASS_LOADER";
        System.out.println("Loaded class: " + loadMe1.getName() + " by " + loader1Name);

        // Example: Loading a core Java class (String), which is loaded by the bootstrap class loader
        Class<?> stringClass = Class.forName("java.lang.String");
        System.out.println("Loaded class: " + stringClass.getName() + " by " + stringClass.getClassLoader());

        // 5. Load a user-defined class
        Class<?> loadMe2 = appClassLoader.loadClass("sample.loader.appclassloaderdemo.LoadMe2");
        ClassLoader loader2 = loadMe1.getClassLoader();
        String loader2Name = (loader2 == appClassLoader) ? loader2.getName() : "NOT_APP_CLASS_LOADER";
        System.out.println("Loaded class: " + loadMe2.getName() + " by " + loader2Name);

        printClassLoader(loadMe1);
        printClassLoader(loadMe2);
        printClassLoader(LoadMe3.class);
        printClassLoader(stringClass);

        // Load the class (this loads & links it, but does not run static block yet)
        Class<?> loadMe4 = appClassLoader.loadClass("sample.loader.appclassloaderdemo.LoadMe4");
        System.out.println("Class object obtained: " + loadMe4.getName());

        // Trigger initialization (static block runs)
        Object instance = loadMe4.getDeclaredConstructor().newInstance();
        loadMe4.getMethod("greet").invoke(instance);
    }

    private static void printClassLoader(Class<?> clazz) {
        System.out.println("Class: " + clazz.getName());

        ClassLoader classLoader = clazz.getClassLoader();
        System.out.println("  loader hierarchy = " + getHierarchy(classLoader));

        if (classLoader != null) {
            System.out.println("  loader name = " + classLoader.getName());
        }

        CodeSource codeSource = clazz.getProtectionDomain().getCodeSource();
//         System.out.println("  code source   = " +
//                 (codeSource == null ? "null" : codeSource.getLocation())); // fixme: null

        Module module = clazz.getModule();
        System.out.println("  module        = " + module.getName());
    }

    private static String getHierarchy(ClassLoader cl) {
        if (cl == null) {
            return "Bootstrap";
        } else {
            return cl.getClass().getName() + " -> " +
                    getHierarchy(cl.getParent());
        }
    }
}

class LoadMe1 {
    static {
        System.out.println("LoadMe1 class has been initialized!");
    }
}

class LoadMe2 {
    static {
        System.out.println("LoadMe2 class has been initialized!");
    }
}

class LoadMe3 {
    static {
        System.out.println("LoadMe3 class has been initialized!");
    }
}

class LoadMe4 {
    static {
        System.out.println("LoadMe4 class has been initialized!");
    }

    public LoadMe4() {
        System.out.println("LoadMe4 instance constructed");
    }

    public void greet() {
        System.out.println("Hello from LoadMe4!");
    }
}
