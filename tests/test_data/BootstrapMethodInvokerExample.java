// javac --patch-module java.base=. -d . BootstrapMethodInvokerExample.java
// java --patch-module java.base=. java.lang.invoke.BootstrapMethodInvokerExample

// This test is put to java.lang.invoke package for calling package-private method
// It's not so nice but a bad test is better than no test
package java.lang.invoke;

import java.util.Arrays;
import java.util.function.BiFunction;

public class BootstrapMethodInvokerExample {
    private static final MethodHandles.Lookup LOOKUP = MethodHandles.lookup();

    public static void main(String[] args) throws Throwable {
        concat();
        System.out.println();
        customBootstrapMethod();
        System.out.println();
        lambdaExpression();
    }

    private static void concat() throws Throwable {
        System.out.println("--- Simulating `invokedynamic` for concat('a','b','c') ---");

        // Step 1: Prepare the parameters for the Bootstrap Method.
        // 1a. The Bootstrap Method Handle (BSM).
        MethodHandle bootstrapMethod = LOOKUP.findStatic(
                StringConcatFactory.class,
                "makeConcatWithConstants",
                MethodType.methodType(
                        CallSite.class,             // the return type
                        MethodHandles.Lookup.class, // the lookup object
                        String.class,               // the name of the method to invoke
                        MethodType.class,           // the method type of the invoked method
                        String.class,               // the recipe for concatenation
                        Object[].class              // the constants to be used in the recipe
                )
        );

        // 1b. The invoked name (usually matches the BSM name, but not required).
        String invokedName = "makeConcatWithConstants";

        // 1c. The invoked method type. The final handle should take three Strings and return one.
        MethodType invokedType = MethodType.methodType(String.class, String.class, String.class, String.class);

        // 1d. The Static Arguments for the BSM.
        // The recipe for "a" + "b" + "c" is "\u0001\u0001\u0001"
        // It means: append arg0, then append arg1, then append arg2.
        // There are no other compile-time constants.
        String recipe = "\u0001\u0001\u0001";
        Object[] staticArgs = new Object[]{recipe};

        // Step 2: Call the package-private `BootstrapMethodInvoker.invoke` method directly.
        // ==============================================================================
        System.out.println("--- Calling BootstrapMethodInvoker.invoke directly ---");

        CallSite callSite = BootstrapMethodInvoker.invoke(
                CallSite.class,                     // The expected resultType
                bootstrapMethod,                    // The BSM handle
                invokedName,                        // The name
                invokedType,                        // The method type
                staticArgs,                         // The static arguments array
                BootstrapMethodInvokerExample.class // The class containing the call site
        );

        // Step 3: Use the final linked MethodHandle.
        // ==========================================
        MethodHandle dynamicInvoker = callSite.dynamicInvoker();
        System.out.println("CallSite's Dynamic Invoker: " + dynamicInvoker);

        System.out.println("--- Executing the final linked MethodHandle ---");
        String finalResult = (String) dynamicInvoker.invokeExact("a", "b", "c");

        System.out.println("Result of invokeExact(\"a\", \"b\", \"c\"): " + finalResult);
    }

    private static void customBootstrapMethod() throws Throwable {
        System.out.println("--- Simulating `invokedynamic` for custom bootstrap method ---");

        MethodHandle bsm = LOOKUP.findStatic(BootstrapMethodInvokerExample.class, "customBootstrap", MethodType.methodType(CallSite.class, MethodHandles.Lookup.class, String.class, MethodType.class));
        MethodType invokedType = MethodType.methodType(int.class, int.class, int.class);

        CallSite cs = BootstrapMethodInvoker.invoke(CallSite.class, bsm, "myCustomAdd", invokedType, null, BootstrapMethodInvokerExample.class);
        MethodHandle invoker = cs.dynamicInvoker();
        int result = (int) invoker.invokeExact(10, 20);
        System.out.println("Custom BSM Result: " + result);
    }

    // =================================================================================
    // SCENARIO 1 BOOTSTRAP METHOD: A custom BSM we write ourselves.
    // =================================================================================
    public static CallSite customBootstrap(MethodHandles.Lookup caller, String name, MethodType type) throws Throwable {
        System.out.println("[BSM]: customBootstrap called!");
        System.out.println("  > invokedName: " + name); // The name from the call site, e.g., "myCustomAdd"
        System.out.println("  > invokedType: " + type); // The type from the call site, e.g., (int, int)int

        // The BSM's job is to find the real method to link to.
        MethodHandle target = caller.findStatic(BootstrapMethodInvokerExample.class, "add", MethodType.methodType(int.class, int.class, int.class));

        System.out.println("  > Linking to target: " + target);

        // We return a ConstantCallSite, which means the link is permanent.
        return new ConstantCallSite(target);
    }

    // =================================================================================
    // Scenario 1: A simple method to be the target of our custom CallSite
    // =================================================================================
    public static int add(int a, int b) {
        System.out.println("Executing add(" + a + ", " + b + ")");
        return a + b;
    }

    private static void lambdaExpression() throws Throwable {
        System.out.println("--- Simulating `invokedynamic` for lambda expression ---");
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        MethodHandle bsm = lookup.findStatic(LambdaMetafactory.class, "metafactory",
                MethodType.methodType(
                        CallSite.class,             // The return type of the BSM
                        MethodHandles.Lookup.class, // The lookup object
                        String.class,               // The name of the method to invoke
                        MethodType.class,           // The method type of the invoked method
                        MethodType.class,           // The type of the factory method
                        MethodHandle.class,         // The method handle to the implementation method
                        MethodType.class            // The "instantiated" type of the lambda (the type we want to create)
                ));

        // The type of the factory method we are creating. It takes no args and returns a BiFunction.
        MethodType invokedType = MethodType.methodType(BiFunction.class);

        // Static arguments for LambdaMetafactory:
        // 1. The signature of the functional interface method: (Object, Object) -> Object
        MethodType samMethodType = MethodType.methodType(Object.class, Object.class, Object.class);
        // 2. A handle to the implementation method.
        MethodHandle implMethod = lookup.findStatic(BootstrapMethodInvokerExample.class, "format", MethodType.methodType(String.class, String.class, Integer.class));
        // 3. The "instantiated" type of the lambda: (String, Integer) -> String
        MethodType instantiatedMethodType = MethodType.methodType(String.class, String.class, Integer.class);

        Object[] staticArgs = new Object[]{samMethodType, implMethod, instantiatedMethodType};

        CallSite cs = BootstrapMethodInvoker.invoke(CallSite.class, bsm, "apply", invokedType, staticArgs, BootstrapMethodInvokerExample.class);

        // The result of a lambda BSM is a "factory" for the lambda. We must invoke it once to get the lambda instance.
        MethodHandle factory = cs.dynamicInvoker();
        BiFunction<String, Integer, String> lambdaInstance = (BiFunction<String, Integer, String>) factory.invokeExact();

        // Now we can use the lambda instance.
        String result = lambdaInstance.apply("User", 123);
        System.out.println("Lambda Result: " + result);
    }

    public static String format(String s, Integer i) {
        System.out.println("Executing lambda body: format('" + s + "', " + i + ")");
        return s + '-' + i;
    }
}
