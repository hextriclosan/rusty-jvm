// build command:
// javac -g -parameters RuntimeInVisibleParameterAnnotations.java

import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;

@Retention(RetentionPolicy.CLASS)
@interface A {}

public class RuntimeInvisibleParameterAnnotations {
    public void someMethod(@A int x) {
    }
}
