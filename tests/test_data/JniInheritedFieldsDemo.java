package samples.javacore.loadlibrary.inheritedfields;

public class JniInheritedFieldsDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native int readInt(Object object, String name, String signature);
    private static native long readLong(Object object, String name, String signature);
    private static native boolean hasField(Object object, String name, String signature);

    public static void main(String[] args) {
        Child child = new Child();
        FirstField firstField = new FirstField();

        System.out.println("inherited=" + readInt(child, "inherited", "I"));
        System.out.println("hidden int=" + readInt(child, "value", "I"));
        System.out.println("hidden long=" + readLong(child, "value", "J"));
        System.out.println("first field ID is non-null=" + hasField(firstField, "first", "I"));
    }
}

class Parent {
    int inherited = 7;
    int value = 11;
}

class Child extends Parent {
    long value = 22;
}

class FirstField {
    int first = 33;
}
