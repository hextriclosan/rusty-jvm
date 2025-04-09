package samples.reflection.mutablecallsiteexample;

import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.invoke.MutableCallSite;

public class MutableCallSiteExample {
    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        MethodHandle target = lookup.findStatic(MutableCallSiteExample.class, "targetMethod1", MethodType.methodType(void.class));
        MutableCallSite callSite = new MutableCallSite(target);
        MethodHandle handle = callSite.dynamicInvoker();

        // Initial invocation (points to targetMethod1)
        handle.invoke();

        // Change the target to targetMethod2
        MethodHandle newTarget = lookup.findStatic(MutableCallSiteExample.class, "targetMethod2", MethodType.methodType(void.class));
        callSite.setTarget(newTarget);

        // Invoke again (now points to targetMethod2)
        handle.invoke();
    }

    private static void targetMethod1() {
        System.out.println("Hello from targetMethod1!");
    }

    private static void targetMethod2() {
        System.out.println("Hello from targetMethod2!");
    }
}
