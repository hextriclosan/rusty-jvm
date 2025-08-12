package samples.reflection.trivial.declaredconstructors;

import java.lang.reflect.Constructor;
import java.lang.reflect.Modifier;
import java.util.Arrays;

public class DeclaredConstructorsExample {
    public static void main(String[] args) throws Exception {

        // BASIC USAGE
        System.out.println("Basic Example Constructors:");
        Constructor<?>[] basicConstructors = BasicExample.class.getDeclaredConstructors();
        printConstructorDetails(basicConstructors);


        // EDGE CASES
        System.out.println("Edge Case Example Constructors:");
        Constructor<?>[] edgeCaseConstructors = EdgeCaseExample.class.getDeclaredConstructors();
        printConstructorDetails(edgeCaseConstructors);

        // HANDLING EXCEPTIONS
        System.out.println("Handle Constructor Exceptions:");
        try {
            Constructor<EdgeCaseExample> constructor = EdgeCaseExample.class.getDeclaredConstructor(Float.class);
            printConstructorDetails(constructor);
        } catch (NoSuchMethodException e) {
            System.out.println("No such constructor found: " + e.getMessage());
        }

        // ACCESS PRIVATE CONSTRUCTOR
        System.out.println("Access Private Constructor:");
        Constructor<BasicExample> privateConstructor = BasicExample.class.getDeclaredConstructor(String.class);
        printConstructorDetails(privateConstructor);
        privateConstructor.setAccessible(true); // Allow access
        BasicExample instance = privateConstructor.newInstance("Test");
        System.out.println("Private instance created: " + instance);

        // ANONYMOUS CLASSES
        System.out.println("Anonymous Class Constructors:");
        Constructor<?>[] anonymousConstructors = new Object() {
        }.getClass().getDeclaredConstructors();
        printConstructorDetails(anonymousConstructors);
    }

    private static void printConstructorDetails(Constructor<?>[] constructors) {
        for (Constructor<?> constructor : constructors) {
            System.out.println("--------------------------------");
            printConstructorDetails(constructor);
        }
    }

    private static void printConstructorDetails(Constructor<?> constructor) {
        System.out.println("Constructor: " + constructor);
        System.out.println("  Parameter types: " + Arrays.toString(constructor.getParameterTypes()));
        System.out.println("  Modifier: " + Modifier.toString(constructor.getModifiers()));
        System.out.println("  Throws: " + Arrays.toString(constructor.getExceptionTypes()));
        //System.out.println(constructor.getAnnotatedReceiverType()); //Native method java/lang/reflect/Executable:getTypeAnnotationBytes0:()[B not found
        //System.out.println(constructor.getAnnotatedReturnType()); //Native method java/lang/reflect/Executable:getTypeAnnotationBytes0:()[B not found
        System.out.println("  Declaring class: " + constructor.getDeclaringClass());
        //printArray(constructor.getGenericParameterTypes());
        //printArray(constructor.getGenericExceptionTypes());
        System.out.println("  Name: " + constructor.getName());
        System.out.println("  Parameter count: " + constructor.getParameterCount());
    }

    private static void printArray(Object[] array) {
        for (Object obj : array) {
            System.out.println(" - " + obj);
        }
    }
}

// Basic class with constructors
class BasicExample {
    public BasicExample() {
    } // Public no-arg constructor

    protected BasicExample(int number) {
    } // Protected constructor

    private BasicExample(String text) {
    } // Private constructor

    @Override
    public String toString() {
        return "BasicExample{}";
    }
}

// Class with edge cases
abstract class EdgeCaseExample {
    EdgeCaseExample() {
    } // Default package-private constructor

    public EdgeCaseExample(String arg) throws IllegalArgumentException {
    } // Constructor with exceptions
}
