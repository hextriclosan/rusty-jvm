package samples.reflection.trivial.declaredconstructors;

import java.lang.reflect.Constructor;
import java.lang.reflect.Modifier;
import java.util.Arrays;

public class DeclaredConstructorsExample {

    // Basic class with constructors
    static class BasicExample {
        public BasicExample() {
        } // Public no-arg constructor

        protected BasicExample(int number) {
        } // Protected constructor

        private BasicExample(String text) {
        } // Private constructor
    }

    // Class with edge cases
    abstract static class EdgeCaseExample {
        EdgeCaseExample() {
        } // Default package-private constructor

        public EdgeCaseExample(String arg) throws IllegalArgumentException {
        } // Constructor with exceptions
    }

    public static void main(String[] args) throws Exception {

        // BASIC USAGE
        System.out.println("Basic Example Constructors:");
        Constructor<?>[] basicConstructors = BasicExample.class.getDeclaredConstructors();
        printConstructorDetails(basicConstructors);


        // EDGE CASES
        System.out.println("Edge Case Example Constructors:");
        Constructor<?>[] edgeCaseConstructors = EdgeCaseExample.class.getDeclaredConstructors();
        printConstructorDetails(edgeCaseConstructors);

        // HANDLING EXCEPTIONS jdk/internal/reflect/DirectMethodHandleAccessor$NativeAccessor:invoke0:(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object; not found
//         System.out.println("Handle Constructor Exceptions:");
//         try {
//             Constructor<EdgeCaseExample> constructor = EdgeCaseExample.class.getDeclaredConstructor(String.class);
//             printConstructorDetails(constructor);
//         } catch (NoSuchMethodException e) {
//             System.out.println("No such constructor found: " + e.getMessage());
//         }

        // ACCESS PRIVATE CONSTRUCTOR jdk/internal/reflect/DirectMethodHandleAccessor$NativeAccessor:invoke0:(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object; not found
//         System.out.println("Access Private Constructor:");
//         Constructor<BasicExample> privateConstructor = BasicExample.class.getDeclaredConstructor(String.class);
//         printConstructorDetails(privateConstructor);
//         privateConstructor.setAccessible(true); // Allow access
//         BasicExample instance = privateConstructor.newInstance("Test");
//         System.out.println("Private instance created: " + instance);

        // ANONYMOUS CLASSES
        System.out.println("Anonymous Class Constructors:");
        Constructor<?>[] anonymousConstructors = new Object() {
        }.getClass().getDeclaredConstructors();
        printConstructorDetails(anonymousConstructors);
    }

    private static void printConstructorDetails(Constructor<?>[] constructors) {
        for (Constructor<?> constructor : constructors) {
            System.out.println("\t----------------------------");
            printConstructorDetails(constructor);
        }
    }

    private static void printConstructorDetails(Constructor<?> constructor) {
        //System.out.println("Constructor: " + constructor); // Native method jdk/internal/reflect/DirectMethodHandleAccessor$NativeAccessor:invoke0:(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object; not found
        System.out.println("\tParameter types: " + Arrays.toString(constructor.getParameterTypes()));
        System.out.println("\tModifier: " + Modifier.toString(constructor.getModifiers()));
        System.out.println("\tThrows: " + Arrays.toString(constructor.getExceptionTypes()));
        //System.out.println(constructor.getAnnotatedReceiverType()); Native method java/lang/reflect/Executable:getTypeAnnotationBytes0:()[B not found
        //System.out.println(constructor.getAnnotatedReturnType()); Native method java/lang/reflect/Executable:getTypeAnnotationBytes0:()[B not found
        System.out.println("\tDeclaring class: " + constructor.getDeclaringClass());
        //printArray(constructor.getGenericParameterTypes());
        //printArray(constructor.getGenericExceptionTypes());
        System.out.println("\tName: " + constructor.getName());
        System.out.println("\tParameter count: " + constructor.getParameterCount());
    }

    private static void printArray(Object[] array) {
        for (Object obj : array) {
            System.out.println(" - " + obj);
        }
    }
}
