package samples.javacore.loadlibrary.arrayops;

import java.util.Arrays;

public class JniPrimitiveArrayCriticalDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native boolean modifyCritical(int[] values);

    public static void main(String[] args) {
        int[] values = { 10, 20, 30 };
        System.out.println("is copy=" + modifyCritical(values));
        System.out.println(Arrays.toString(values));
    }
}
