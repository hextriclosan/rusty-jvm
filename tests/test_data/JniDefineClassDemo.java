package samples.javacore.loadlibrary.classops;

public class JniDefineClassDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native Class<?> define(String name, ClassLoader loader, byte[] bytecode);

    public static void main(String[] args) throws Exception {
        String resource = "/samples/javacore/loadlibrary/classops/DefinedByJni.class";
        byte[] bytecode = JniDefineClassDemo.class.getResourceAsStream(resource).readAllBytes();
        Class<?> defined = define(null, JniDefineClassDemo.class.getClassLoader(), bytecode);
        Object instance = defined.getDeclaredConstructor().newInstance();
        System.out.println(defined.getName());
        System.out.println(defined.getDeclaredMethod("message").invoke(instance));
    }
}

class DefinedByJni {
    public String message() {
        return "defined through JNI";
    }
}
