// javac --patch-module java.base=. System.java
package java.lang;

public final class System {
    private System() {
    }

    public static native long currentTimeMillis();
}
