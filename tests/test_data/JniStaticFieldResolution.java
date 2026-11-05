package samples.jni.staticfields;

// none of these are constant variables, so they are set when their interface is initialized
interface Constants {
    int CONSTANT = Integer.parseInt("7");
    StringBuilder SHARED = new StringBuilder("shared");
}

interface MoreConstants extends Constants {
    long MORE = Long.parseLong("8");
}

class StaticFieldParent implements MoreConstants {
    static int value = 17;
    static int shadowed = 1;
}

class StaticFieldChild extends StaticFieldParent {
    // hides the parent's int field of the same name
    static long value = 9_000_000_000L;
    // an instance field, which GetStaticFieldID passes over
    int shadowed = 2;
}

public class JniStaticFieldResolution {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native int getInt(Class<?> target, String name);
    private static native long getLong(Class<?> target, String name);
    private static native void setInt(Class<?> target, String name, int value);
    private static native Object getObject(Class<?> target, String name, String signature);

    public static void main(String[] args) {
        Class<?> child = StaticFieldChild.class;
        print("int value of the parent", () -> getInt(child, "value"));
        print("long value of the child", () -> getLong(child, "value"));
        print("static field behind an instance field", () -> getInt(child, "shadowed"));
        // initializing a class doesn't initialize its interfaces
        print("field of an uninitialized superinterface", () -> getInt(child, "CONSTANT"));
        print("field of an uninitialized interface", () -> getLong(child, "MORE"));
        print("initialize the interfaces", () -> MoreConstants.MORE + " " + Constants.SHARED);
        print("field of a superinterface", () -> getInt(child, "CONSTANT"));
        print("field of an interface", () -> getLong(child, "MORE"));
        print("object field of a superinterface", () -> getObject(child, "SHARED", "Ljava/lang/StringBuilder;"));
        print("missing field", () -> getInt(child, "missing"));
        print("wrong type", () -> getLong(child, "CONSTANT"));
        print("malformed signature", () -> getObject(child, "value", "Lnot/a/Signature"));
        print("set the parent's field", () -> {
            setInt(child, "value", 42);
            return StaticFieldParent.value + " " + StaticFieldChild.value;
        });
        print("set a missing field", () -> {
            setInt(child, "missing", 1);
            return "done";
        });
    }

    private static void print(String label, java.util.function.Supplier<Object> action) {
        try {
            System.out.println(label + ": " + action.get());
        } catch (NoSuchFieldError e) {
            System.out.println(label + ": " + e);
        }
    }
}
