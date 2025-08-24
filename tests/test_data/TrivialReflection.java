package samples.reflection.trivial;

public class TrivialReflection {
    public static void main(String[] args) {
        print(Class.class);
        print(Object.class);
        print(Examinee.class);
        print(ExamineeInterface.class);
        // print(Rcd.class); // Native Call Error: Native method java/lang/Class:isRecord0:()Z
        print(Enm.class);
        print(int.class);
    }

    private static void print(Class<?> clazz) {
        System.out.println("Class: " + clazz);
        System.out.println("  modifiers: " + clazz.getModifiers());
        System.out.println("  isPrimitive: " + clazz.isPrimitive());
        System.out.println("  isArray: " + clazz.isArray());
        System.out.println("  isInterface: " + clazz.isInterface());
        System.out.println("  isEnum: " + clazz.isEnum());
        System.out.println("  isAnnotation: " + clazz.isAnnotation());
        System.out.println("  isSynthetic: " + clazz.isSynthetic());
        System.out.println("  isRecord: " + clazz.isRecord());
        System.out.println("  isEnumeration: " + clazz.isEnum());
    }
}

abstract class Examinee {
}

interface ExamineeInterface {
}

record Rcd() {
}

enum Enm {
}
