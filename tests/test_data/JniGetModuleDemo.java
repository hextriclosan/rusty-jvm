package samples.javacore.loadlibrary.classops;

public class JniGetModuleDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native Module getModule(Class<?> target);

    public static void main(String[] args) {
        printModule(String.class);
        printModule(int.class);
        printModule(String[].class);
        printModule(JniGetModuleDemo[].class);
        printModule(JniGetModuleDemo.class);
    }

    private static void printModule(Class<?> target) {
        Module module = getModule(target);
        System.out.println(target.getName() + "=" + module.getName());
    }
}
