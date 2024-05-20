// build command:
// javac -g -parameters RuntimeInvisibleAnnotationUsage.java

import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;

@Retention(RetentionPolicy.CLASS)
@interface RuntimeInvisibleAnnotation {
    String value();
}

@RuntimeInvisibleAnnotation(value = "This is a runtime invisible annotation")
public interface RuntimeInvisibleAnnotationUsage {
}
