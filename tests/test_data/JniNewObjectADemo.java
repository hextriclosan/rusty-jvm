package samples.javacore.loadlibrary.objectops;

public class JniNewObjectADemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native Sample create(Class<?> target, int number, String text);

    public static void main(String[] args) {
        Sample sample = create(Sample.class, 42, "value");
        System.out.println(sample.number + ":" + sample.text);
    }
}

class Sample {
    final int number;
    final String text;

    Sample(int number, String text) {
        this.number = number;
        this.text = text;
    }
}
