package samples.javacore.loadlibrary.classops;

public class JniGetModuleDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native Module getModule(Class<?> target);

    public static void main(String[] args) {
        printModule(String.class);
        printModule(int.class);
        printModule(int[].class);
        printModule(String[].class);
        printModule(String[][].class);
        printModule(JniGetModuleDemo[].class);
        printModule(JniGetModuleDemo.class);
    }

    private static void printModule(Class<?> target) {
        Module jniModule = getModule(target);
        Module javaModule = target.getModule();
        System.out.println(target.getName() + "=" + jniModule.getName()
                + " sameAsJava=" + (jniModule == javaModule));
    }
}
