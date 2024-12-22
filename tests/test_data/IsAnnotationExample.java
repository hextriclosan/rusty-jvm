package samples.reflection.trivial.isannotationexample;

import java.lang.annotation.ElementType;
import java.lang.annotation.Inherited;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

public class IsAnnotationExample {
    public static void main(String[] args) {
        // Trivial Case: Simple annotation
        System.out.print("@MyAnnotation is annotation: ");
        System.out.println(MyAnnotation.class.isAnnotation()); // true

        // Case 1: Non-annotation class
        System.out.print("String.class is annotation: ");
        System.out.println(String.class.isAnnotation()); // false

        // Case 2: Annotation subclass (edge case)
        System.out.print("@InheritedAnnotation is annotation: ");
        System.out.println(InheritedAnnotation.class.isAnnotation()); // true
        System.out.print("@MyAnnotation subclass is annotation: ");
        System.out.println(SubAnnotation.class.isAnnotation()); // true

        // Case 3: Runtime vs. compile-time retention
        System.out.print("@SourceAnnotation is annotation: ");
        System.out.println(SourceAnnotation.class.isAnnotation()); // true

        // Case 4: Arrays and primitives
        System.out.print("Annotation array is annotation: ");
        System.out.println(MyAnnotation[].class.isAnnotation()); // false
        System.out.print("int.class is annotation: ");
        System.out.println(int.class.isAnnotation()); // false

        // Case 5: Proxy class (edge case)
//         Object proxy = java.lang.reflect.Proxy.newProxyInstance( //not yet implemented: INVOKEDYNAMIC
//                 IsAnnotationExample.class.getClassLoader(),
//                 new Class<?>[]{MyAnnotation.class},
//                 (proxyInstance, method, methodArgs) -> null
//         );
//         System.out.print("Proxy of @MyAnnotation is annotation: ");
//         System.out.println(proxy.getClass().isAnnotation()); // false

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
