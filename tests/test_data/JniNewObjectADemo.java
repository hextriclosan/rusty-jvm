package samples.javacore.loadlibrary.objectops;

public class JniNewObjectADemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native Object create(Class<?> target, Class<?> constructorOwner, int number, String text);

    public static void main(String[] args) {
        print("plain", ConstructedSample.class, ConstructedSample.class, 42, "value");
        print("super constructor", DerivedSample.class, ConstructedSample.class, 7, "base");
        print("throwing constructor", ConstructedSample.class, ConstructedSample.class, -1, "bad");
        print("abstract", AbstractConstructedSample.class, AbstractConstructedSample.class, 1, "abstract");
        print("null", null, ConstructedSample.class, 1, "null");
    }

    private static void print(String label, Class<?> target, Class<?> constructorOwner, int number, String text) {
        try {
            System.out.println(label + " -> " + create(target, constructorOwner, number, text));
        } catch (Exception e) {
            System.out.println(label + " -> " + e);
        }
    }
}

class ConstructedSample {
    final int number;
    final String text;

    ConstructedSample(int number, String text) {
        if (number < 0) {
            throw new IllegalArgumentException("negative: " + number);
        }
        this.number = number;
        this.text = text;
    }

    @Override
    public String toString() {
        return getClass().getSimpleName() + " " + number + ":" + text;
    }
}

class DerivedSample extends ConstructedSample {
    int extra = 99;

    DerivedSample(int number, String text) {
        super(number, text);
    }

    @Override
    public String toString() {
        return super.toString() + " extra=" + extra;
    }
}

abstract class AbstractConstructedSample {
    AbstractConstructedSample(int number, String text) {}
}
