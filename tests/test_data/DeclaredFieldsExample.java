package samples.reflection.trivial.declaredfields;

import java.lang.reflect.Field;
import java.lang.reflect.Modifier;
import java.lang.reflect.Type;
import java.util.Map;

public class DeclaredFieldsExample {
    public static void main(String[] args) {
        print(Examinee.class);
        print(Examinee.MyInner.class);
    }

    private static void print(Class<?> clazz) {
        System.out.println("Class: " + clazz.getName());
        System.out.println("All declared fields:");
        for (Field field : clazz.getDeclaredFields()) {
            print(field);
        }
        System.out.println("Public only fields:");
        for (Field field : clazz.getFields()) {
            print(field);
        }
    }

    private static void print(Field field) {
        System.out.println("Information about field: " + field.getName());
        System.out.println("------------------------------------------------");
        System.out.println("String representation: " + field);
//             System.out.println("Generic String representation: " + field.toGenericString()); // https://github.com/hextriclosan/rusty-jvm/issues/386
        System.out.println("Class: " + field.getDeclaringClass());
        System.out.println("Modifiers: " + Modifier.toString(field.getModifiers()));

        Class<?> type = field.getType();
        System.out.println("Type: " + type);

//             Type genericType = field.getGenericType();
//             System.out.println("Generic Parameter Type: " + genericType.getTypeName()); // https://github.com/hextriclosan/rusty-jvm/issues/386

        System.out.println("Is Synthetic: " + field.isSynthetic());
//             System.out.println("Annotations: "); // https://github.com/hextriclosan/rusty-jvm/issues/386
//             for (Object annotation : field.getAnnotations()) {
//                 System.out.println(" - " + String.valueOf(annotation));
//             }

        System.out.println();
    }
}

class Examinee {
    public int publicField;
    public static Object publicStaticField;
    protected String protectedField;
    @Deprecated private double privateField;
    final int[] packagePrivateFinalField = null;
    static final String staticFinalField = "static final field";
    static Map<String, Integer> staticNonFinalField;
    transient String transientField;

    class MyInner {
        // The compiler generates a synthetic field 'this$0' in this class
        // to hold the reference to the enclosing 'Examinee' instance.

        // This access forces the compiler to generate 'this$0'
        int i = publicField;
    }
}
