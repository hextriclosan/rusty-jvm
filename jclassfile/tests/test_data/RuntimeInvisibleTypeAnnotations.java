// build command:
// javac -g -parameters RuntimeInvisibleTypeAnnotations.java

import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

@Target(ElementType.TYPE_USE)
@Retention(RetentionPolicy.CLASS)
@interface A {}

public class RuntimeInvisibleTypeAnnotations {
    public void someMethod(@A int x) {
    }
}
