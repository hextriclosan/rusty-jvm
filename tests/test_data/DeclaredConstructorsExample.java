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
        //System.out.print("Constructor: ");
        //System.out.println(constructor); // Native method jdk/internal/reflect/DirectMethodHandleAccessor$NativeAccessor:invoke0:(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object; not found
        System.out.print("\tParameter types: ");
        System.out.println(Arrays.toString(constructor.getParameterTypes()));
        System.out.print("\tModifier: ");
        System.out.println(Modifier.toString(constructor.getModifiers()));
        System.out.print("\tThrows: ");
        System.out.println(Arrays.toString(constructor.getExceptionTypes()));

        //System.out.println(constructor.getAnnotatedReceiverType()); Native method java/lang/reflect/Executable:getTypeAnnotationBytes0:()[B not found
        //System.out.println(constructor.getAnnotatedReturnType()); Native method java/lang/reflect/Executable:getTypeAnnotationBytes0:()[B not found
        System.out.print("\tDeclaring class: ");
        System.out.println(constructor.getDeclaringClass());
        //printArray(constructor.getGenericParameterTypes());
        //printArray(constructor.getGenericExceptionTypes());
        System.out.print("\tName: ");
        System.out.println(constructor.getName());
        System.out.print("\tParameter count: ");
        System.out.println(constructor.getParameterCount());
    }

    private static void printArray(Object[] array) {
        for (Object obj : array) {
            System.out.print(" - ");
            System.out.println(obj);
        }
    }
}
