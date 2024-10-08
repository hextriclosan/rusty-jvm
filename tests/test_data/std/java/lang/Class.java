// javac --patch-module java.base=. Class.java

package java.lang;

public final class Class<T> {
    public native int getModifiers();
}
