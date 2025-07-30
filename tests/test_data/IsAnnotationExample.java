package samples.reflection.trivial.isannotationexample;

import java.lang.annotation.ElementType;
import java.lang.annotation.Inherited;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

public class IsAnnotationExample {
    public static void main(String[] args) {
        // Trivial Case: Simple annotation
        System.out.println("@MyAnnotation is annotation: " + MyAnnotation.class.isAnnotation()); // true

        // Case 1: Non-annotation class
        System.out.println("String.class is annotation: " + String.class.isAnnotation()); // false

        // Case 2: Annotation subclass (edge case)
        System.out.println("@InheritedAnnotation is annotation: " + InheritedAnnotation.class.isAnnotation()); // true
        System.out.println("@MyAnnotation subclass is annotation: " + SubAnnotation.class.isAnnotation()); // true

        // Case 3: Runtime vs. compile-time retention
        System.out.println("@SourceAnnotation is annotation: " + SourceAnnotation.class.isAnnotation()); // true

        // Case 4: Arrays and primitives
        System.out.println("Annotation array is annotation: " + MyAnnotation[].class.isAnnotation()); // false
        System.out.println("int.class is annotation: " + int.class.isAnnotation()); // false

        // Case 5: Proxy class (edge case)
//         Object proxy = java.lang.reflect.Proxy.newProxyInstance( //not yet implemented: INVOKEDYNAMIC
//                 IsAnnotationExample.class.getClassLoader(),
//                 new Class<?>[]{MyAnnotation.class},
//                 (proxyInstance, method, methodArgs) -> null
//         );
//         System.out.println("Proxy of @MyAnnotation is annotation: " + proxy.getClass().isAnnotation()); // false

    }

    // Annotation definitions
    @Retention(RetentionPolicy.RUNTIME)
    @Target(ElementType.TYPE)
    @interface MyAnnotation {
    }

    @Retention(RetentionPolicy.CLASS)
    @Target(ElementType.TYPE)
    @interface SourceAnnotation {
    }

    @MyAnnotation
    @interface SubAnnotation {
    }

    @Inherited
    @Retention(RetentionPolicy.RUNTIME)
    @Target(ElementType.TYPE)
    @interface InheritedAnnotation {
    }
}
