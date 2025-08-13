package samples.reflection.trivial.synthetic.classes;

import java.lang.reflect.Modifier;
import java.util.IdentityHashMap;
import java.util.Map;

public class SyntheticPrimitiveClasses {
    public static void main(String[] args) {
        print(int.class);
        print(double.class);
        print(boolean.class);
        print(char.class);
        print(byte.class);
        print(short.class);
        print(long.class);
        print(float.class);
        print(void.class);

        System.out.println("=== Proof of materialization ===");
        Class<?> c1 = int.class;
        Class<?> c2 = Integer.TYPE;
        System.out.println("c1 == c2: " + (c1 == c2)); // should be true
        System.out.println("System.identityHashCode(c1) == System.identityHashCode(c2): " + (System.identityHashCode(c1) == System.identityHashCode(c2)));

        // Store in IdentityHashMap (proves reference identity)
        Map<Class<?>, String> map = new IdentityHashMap<>();
        map.put(c1, "Primitive int");
        System.out.println("Lookup by c2: " + map.get(c2)); // works because same object

        // Show it's an Object
        Object o = c1;
        System.out.println("o.getClass(): " + o.getClass()); // class java.lang.Class
    }

    private static void print(Class<?> clazz) {
        System.out.println("==== " + clazz + " ====");
        System.out.println("Name: " + clazz.getName());
        System.out.println("Type name: " + clazz.getTypeName());
        System.out.println("Simple name: " + clazz.getSimpleName());
        System.out.println("Modifiers: " + clazz.getModifiers() +
                " (" + Modifier.toString(clazz.getModifiers()) + ")");
        System.out.println("isPrimitive: " + clazz.isPrimitive());
        System.out.println("isSynthetic: " + clazz.isSynthetic());
        System.out.println("Package: " + clazz.getPackage());
        System.out.println("Declaring class: " + clazz.getDeclaringClass());

        // Wrapper type info
        Class<?> wrapper = getWrapperType(clazz);
        System.out.println("Wrapper type: " + wrapper);
        System.out.println();
    }

    private static Class<?> getWrapperType(Class<?> primitive) {
        if (!primitive.isPrimitive()) {
            return primitive;
        }
        if (primitive == int.class) return Integer.class;
        if (primitive == double.class) return Double.class;
        if (primitive == boolean.class) return Boolean.class;
        if (primitive == char.class) return Character.class;
        if (primitive == byte.class) return Byte.class;
        if (primitive == short.class) return Short.class;
        if (primitive == long.class) return Long.class;
        if (primitive == float.class) return Float.class;
        if (primitive == void.class) return Void.class;
        return null;
    }
}
