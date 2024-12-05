// javac --add-exports java.base/jdk.internal.reflect=ALL-UNNAMED  -d . ReflectionGetCallerClassExample.java
package samples.jdkinternal.reflection.getcallerclass;

import jdk.internal.reflect.Reflection;

public class ReflectionGetCallerClassExample {
    public static void main(String[] args) {
        Class<?> caller = new Caller().invokeNested();

        int result = caller == ReflectionGetCallerClassExample.class ? 1 : 0;
        System.out.println(result);
    }
}

class Caller {
    public Class<?> invokeNested() {
        return intermediateMethod();
    }

    private Class<?> intermediateMethod() {
        return finalMethod();
    }

    private Class<?> finalMethod() {
        return getCallerInfo();
    }

    private Class<?> getCallerInfo() {
        return Reflection.getCallerClass();
    }
}
