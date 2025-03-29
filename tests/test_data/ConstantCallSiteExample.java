package samples.reflection.constantcallsiteexample;

import java.lang.invoke.CallSite;
import java.lang.invoke.ConstantCallSite;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;

public class ConstantCallSiteExample {
    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        MethodHandle target = lookup.findStatic(ConstantCallSiteExample.class, "targetMethod", MethodType.methodType(void.class));
        CallSite callSite = new ConstantCallSite(target);

        MethodHandle handle = callSite.dynamicInvoker();
        handle.invokeExact();
    }

    private static void targetMethod() {
        System.out.println("Hello from CallSite!");
    }
}
