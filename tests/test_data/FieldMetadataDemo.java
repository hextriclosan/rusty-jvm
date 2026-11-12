package samples.reflection.fields;

import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;
import java.lang.annotation.ElementType;
import java.lang.reflect.Field;
import java.lang.reflect.TypeVariable;
import java.util.List;
import java.util.Map;

public class FieldMetadataDemo<T extends Number> {
    @Retention(RetentionPolicy.RUNTIME)
    @Target(ElementType.FIELD)
    public @interface Mark {
        String value() default "default";
    }

    @Retention(RetentionPolicy.CLASS)
    @Target(ElementType.FIELD)
    public @interface Hidden {}

    @Mark("names") @Hidden
    public List<String> names;
    @Mark
    public T variable;
    public Map<? extends Number, List<T[]>> nested;
    public int plain;

    public static void main(String[] args) throws Exception {
        for (String name : new String[] {"names", "variable", "nested", "plain"}) {
            Field field = FieldMetadataDemo.class.getDeclaredField(name);
            System.out.println(field.toGenericString());
            System.out.println(field.getGenericType().getTypeName());
            Mark mark = field.getAnnotation(Mark.class);
            System.out.println(mark == null ? "unmarked" : mark.value());
            System.out.println("annotations=" + field.getAnnotations().length);
            System.out.println("declared=" + field.getDeclaredAnnotations().length);
            if (field.isAnnotationPresent(Hidden.class)) {
                throw new AssertionError("invisible annotation");
            }
        }
        TypeVariable<?> type = (TypeVariable<?>) FieldMetadataDemo.class.getDeclaredField("variable").getGenericType();
        if (type.getGenericDeclaration() != FieldMetadataDemo.class || type.getBounds()[0] != Number.class) {
            throw new AssertionError("type variable bounds");
        }
        System.out.println("field-metadata-ok");
    }
}
