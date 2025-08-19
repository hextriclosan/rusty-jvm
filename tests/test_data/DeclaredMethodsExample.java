package samples.reflection.trivial.declaredmethods;

import java.io.IOError;
import java.lang.reflect.Method;
import java.lang.reflect.Modifier;
import java.lang.reflect.Type;
import java.util.Arrays;

public class DeclaredMethodsExample {
    @Deprecated
    private native String[] sampleMethod(String arg, int count, double... doubles) throws IOError, NullPointerException;

    public static void main(String[] args) {
        Class<?> clazz = DeclaredMethodsExample.class;

        System.out.println("Class: " + clazz.getName());
        System.out.println("All declared method:");
        print(clazz.getDeclaredMethods());

        System.out.println();

        System.out.println("Public only methods:");
        print(clazz.getMethods());
    }

    private static void print(Method[] methods) {
        Arrays.sort(methods, (m1, m2) -> m1.toString().compareTo(m2.toString())); // change to Comparator.comparing(Method::toString) // VM execution failed: Execution Error: Error running invokedynamic for class java/util/Comparator on index 61: Exception thrown
        for (Method method : methods) {
            print(method);
        }
    }

    private static void print(Method method) {
        System.out.println("Information about method:" + method.getName());
        System.out.println("------------------------------------------------");
        System.out.println("String representation:" + method);
        // System.out.println("Generic String representation: " + method.toGenericString()); // Generic String representation: public final native java.lang.Class<?> java.lang.Object.getClass() vs Generic String representation: public final native java.lang.Class java.lang.Object.getClass()
        System.out.println("Class:" + method.getDeclaringClass());
        System.out.println("Return Type:" + method.getReturnType());
        System.out.println("Modifiers:" + Modifier.toString(method.getModifiers()));
        System.out.println("Parameter Count:" + method.getParameterCount());

        Class<?>[] parameterTypes = method.getParameterTypes();
        System.out.println("Parameter Types:");
        for (Class<?> paramType : parameterTypes) {
            System.out.println(" - " + paramType.getName());
        }

        Type[] genericParameterTypes = method.getGenericParameterTypes();
        System.out.println("Generic Parameter Types:");
        for (Type genericType : genericParameterTypes) {
            System.out.println(" - " + genericType.getTypeName());
        }

        System.out.println("Is Synthetic:" + method.isSynthetic());
        System.out.println("Is Default:" + method.isDefault());
        System.out.println("Is Bridge:" + method.isBridge());
        Class<?>[] exceptionTypes = method.getExceptionTypes();
        System.out.println("Exception Types:");
        for (Class<?> exceptionType : exceptionTypes) {
            System.out.println(" - " + exceptionType.getName());
        }

        Type[] genericExceptionTypes = method.getGenericExceptionTypes();
        System.out.println("Generic Exception Types:");
        for (Type genericExceptionType : genericExceptionTypes) {
            System.out.println(" - " + genericExceptionType.getTypeName());
        }

        System.out.println("Is VarArgs:" + method.isVarArgs());
        // System.out.println("Annotations: ");
        // for (Object annotation : method.getAnnotations()) { // Exception in thread "system" java.lang.InternalError: Proxy is not supported until module system is fully initialized
        //     System.out.println(" - " + String.valueOf(annotation));
        /// }
        // System.out.println("Generic Return Type:" + method.getGenericReturnType()); // Generic Return Type:java.lang.Class<?> vs Generic Return Type:class java.lang.Class

        System.out.println();
    }
}
