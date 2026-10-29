package samples.javacore.loadlibrary.objectops;

public class JniNewObjectADemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native ConstructedSample create(Class<?> target, int number, String text);

    public static void main(String[] args) {
        ConstructedSample sample = create(ConstructedSample.class, 42, "value");
        System.out.println(sample.number + ":" + sample.text);
    }
}

class ConstructedSample {
    final int number;
    final String text;

    ConstructedSample(int number, String text) {
        this.number = number;
        this.text = text;
    }
}
