package samples.javacore.loadlibrary.nio;

import java.nio.ByteBuffer;

public class JniDirectBufferDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native ByteBuffer create(long capacity);
    private static native long address(ByteBuffer buffer);
    private static native long capacity(ByteBuffer buffer);
    private static native void release(ByteBuffer buffer);

    public static void main(String[] args) {
        ByteBuffer buffer = create(4);
        buffer.put(0, (byte) 12);
        buffer.put(3, (byte) 34);
        System.out.println("direct=" + buffer.isDirect());
        System.out.println("capacity=" + capacity(buffer));
        System.out.println("address nonzero=" + (address(buffer) != 0));
        System.out.println("values=" + buffer.get(0) + "," + buffer.get(3));
        release(buffer);
    }
}
