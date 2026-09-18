package samples.javacore.loadlibrary.threadops;

public class JniIsVirtualThreadDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native boolean isVirtual(Thread thread);

    public static void main(String[] args) {
        Thread platform = new Thread();
        Thread virtual = Thread.ofVirtual().unstarted(() -> {});
        System.out.println("platform=" + isVirtual(platform));
        System.out.println("virtual=" + isVirtual(virtual));
        System.out.println("null=" + isVirtual(null));
    }
}
