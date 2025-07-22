package samples.reflection.stringconcat;

import java.lang.invoke.CallSite;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.invoke.StringConcatFactory;

public class StringConcatFactoryExample {
    public static void main(String[] args) throws Throwable {
        // Create a method handle using StringConcatFactory
        MethodHandles.Lookup lookup = MethodHandles.lookup();

        // Define the method type for string concatenation (3 String arguments -> String)
        MethodType methodType = MethodType.methodType(String.class, String.class, String.class, String.class);

        // Create a call site using StringConcatFactory (mimicking invokedynamic behavior)
        CallSite callSite = StringConcatFactory.makeConcat(lookup, "concat", methodType);

        // Return the dynamically generated method handle
        MethodHandle handle = callSite.dynamicInvoker();

        // Invoke the dynamically generated method handle
        String result = (String) handle.invokeExact("a", "b", "c");
        System.out.println(result);
    }
}
