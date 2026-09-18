package samples.javacore.varhandle.fullfence;

import java.lang.invoke.VarHandle;

public class VarHandleFullFence {
    public static void main(String[] args) {
        VarHandle.fullFence();
        System.out.println("completed");
    }
}
