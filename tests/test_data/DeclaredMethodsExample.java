// javac -XDstringConcat=inline  -d . DeclaredMethodsExample.java
package samples.reflection.trivial.declaredmethods;

import java.io.IOError;
import java.lang.reflect.Method;
import java.lang.reflect.Modifier;
import java.lang.reflect.Type;

public class DeclaredMethodsExample {
    @Deprecated
    private native String[] sampleMethod(String arg, int count, double... doubles) throws IOError, NullPointerException;

    public static void main(String[] args) {
        Method[] methods = DeclaredMethodsExample.class.getDeclaredMethods();
        for (Method method : methods) {
            System.out.println("Information about method:" + method.getName());
            System.out.println("------------------------------------------------");
//             System.out.println("String representation: " + method); // Native method java/lang/Class:getConstantPool:()Ljdk/internal/reflect/ConstantPool; not found;;; Native method jdk/internal/reflect/DirectMethodHandleAccessor$NativeAccessor:invoke0:(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object; not found\n"
//             System.out.println("Generic String representation: " + method.toGenericString()); // Native method java/lang/Class:getConstantPool:()Ljdk/internal/reflect/ConstantPool; not found;;; Native method jdk/internal/reflect/DirectMethodHandleAccessor$NativeAccessor:invoke0:(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object; not found\n"
            System.out.println("Class:" + method.getDeclaringClass());
            System.out.println("Return Type:" + method.getReturnType());
            System.out.println("Modifiers:" + Modifier.toString(method.getModifiers()));
            System.out.println("Parameter Count:" + method.getParameterCount());

            Class<?>[] parameterTypes = method.getParameterTypes();
            System.out.println("Parameter Types:");
            for (Class<?> paramType : parameterTypes) {
                System.out.println('\t' + paramType.getName());
            }

            Type[] genericParameterTypes = method.getGenericParameterTypes();
            System.out.println("Generic Parameter Types:");
            for (Type genericType : genericParameterTypes) {
                System.out.println('\t' + genericType.getTypeName());
            }

            System.out.println("Is Synthetic:" + method.isSynthetic());
            System.out.println("Is Default:" + method.isDefault());
            System.out.println("Is Bridge:" + method.isBridge());
            Class<?>[] exceptionTypes = method.getExceptionTypes();
            System.out.println("Exception Types:");
            for (Class<?> exceptionType : exceptionTypes) {
                System.out.println('\t' + exceptionType.getName());
            }

            Type[] genericExceptionTypes = method.getGenericExceptionTypes();
            System.out.println("Generic Exception Types:");
            for (Type genericExceptionType : genericExceptionTypes) {
                System.out.println('\t' + genericExceptionType.getTypeName());
            }

            System.out.println("Is VarArgs:" + method.isVarArgs());
//             System.out.println("Annotations: ");
//             for (Object annotation : method.getAnnotations()) { // Native method java/lang/Class:getConstantPool:()Ljdk/internal/reflect/ConstantPool; not found
//                 System.out.println('\t' + String.valueOf(annotation));
//             }
            System.out.println("Generic Return Type:" + method.getGenericReturnType());

            System.out.println();
        }
    }
}
