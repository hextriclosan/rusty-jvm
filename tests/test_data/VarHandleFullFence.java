package samples.javacore.varhandle.fullfence;

import java.lang.invoke.VarHandle;

public class VarHandleFullFence {
    public static void main(String[] args) {
        // each fence reaches the native Unsafe.fullFence, directly or through Unsafe.loadFence and storeFence
        VarHandle.fullFence();
        System.out.println("fullFence");
        VarHandle.acquireFence();
        System.out.println("acquireFence");
        VarHandle.releaseFence();
        System.out.println("releaseFence");
        VarHandle.loadLoadFence();
        System.out.println("loadLoadFence");
        VarHandle.storeStoreFence();
        System.out.println("storeStoreFence");
    }
}
