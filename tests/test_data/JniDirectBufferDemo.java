package samples.javacore.loadlibrary.nio;

import java.nio.ByteBuffer;

public class JniDirectBufferDemo {
    static {
        System.loadLibrary("jni_test_lib");
    }

    private static native ByteBuffer create(long capacity);
    private static native ByteBuffer createUnbacked(long capacity);
    private static native long address(Object buffer);
    private static native long capacity(Object buffer);
    private static native void fill(ByteBuffer buffer, byte value);
    private static native void release(ByteBuffer buffer);

    public static void main(String[] args) {
        ByteBuffer buffer = create(4);
        buffer.put(0, (byte) 12);
        buffer.put(3, (byte) 34);
        System.out.println("direct=" + buffer.isDirect());
        System.out.println("capacity=" + capacity(buffer));
        System.out.println("address nonzero=" + (address(buffer) != 0));
        System.out.println("values=" + buffer.get(0) + "," + buffer.get(3));
        fill(buffer, (byte) 7);
        System.out.println("filled=" + buffer.get(0) + "," + buffer.get(3));
        System.out.println("int view capacity=" + capacity(buffer.asIntBuffer()));
        release(buffer);

        ByteBuffer heap = ByteBuffer.allocate(8);
        System.out.println("heap address=" + address(heap) + " capacity=" + capacity(heap));
        System.out.println("string address=" + address("text") + " capacity=" + capacity("text"));

        System.out.println("empty capacity=" + capacity(createUnbacked(0)));
        try {
            createUnbacked(-1);
            System.out.println("negative capacity accepted");
        } catch (IllegalArgumentException e) {
            System.out.println("negative capacity: " + e.getMessage());
        }
    }
}
