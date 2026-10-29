package samples.javacore.loadlibrary.classops;

import java.nio.charset.StandardCharsets;

public class JniDefineClassDemo {
    private static final String PACKAGE = "samples/javacore/loadlibrary/classops/";

    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native Class<?> define(String name, ClassLoader loader, byte[] bytecode);

    public static void main(String[] args) throws Exception {
        ClassLoader loader = JniDefineClassDemo.class.getClassLoader();

        Class<?> unnamed = define(null, loader, renamedTemplate("DefinedRenamed1"));
        describe(unnamed, loader);

        Class<?> named = define(PACKAGE + "DefinedRenamed2", loader, renamedTemplate("DefinedRenamed2"));
        describe(named, loader);

        attempt("wrong name", PACKAGE + "SomethingElse", loader, renamedTemplate("DefinedRenamed3"));
        attempt("duplicate", null, loader, renamedTemplate("DefinedRenamed1"));
        attempt("malformed", null, loader, new byte[] {1, 2, 3, 4});
    }

    /// Returns the bytes of `DefinedTemplate` with its name replaced by a same-length one, so the
    /// result describes a class that is not on the classpath.
    private static byte[] renamedTemplate(String simpleName) throws Exception {
        String resource = "/" + PACKAGE + "DefinedTemplate.class";
        byte[] bytes = JniDefineClassDemo.class.getResourceAsStream(resource).readAllBytes();
        byte[] from = (PACKAGE + "DefinedTemplate").getBytes(StandardCharsets.UTF_8);
        byte[] to = (PACKAGE + simpleName).getBytes(StandardCharsets.UTF_8);
        for (int i = 0; i + from.length <= bytes.length; i++) {
            if (java.util.Arrays.equals(bytes, i, i + from.length, from, 0, from.length)) {
                System.arraycopy(to, 0, bytes, i, to.length);
            }
        }
        return bytes;
    }

    private static void describe(Class<?> defined, ClassLoader loader) throws Exception {
        Object instance = defined.getDeclaredConstructor().newInstance();
        System.out.println(defined.getName() + " loader=" + (defined.getClassLoader() == loader)
                + " message=" + defined.getDeclaredMethod("message").invoke(instance));
    }

    private static void attempt(String label, String name, ClassLoader loader, byte[] bytecode) {
        try {
            System.out.println(label + " -> " + define(name, loader, bytecode));
        } catch (LinkageError e) {
            System.out.println(label + " -> " + e.getClass().getName());
        }
    }
}

class DefinedTemplate {
    public String message() {
        return "defined through JNI";
    }
}
