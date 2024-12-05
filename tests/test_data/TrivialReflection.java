package samples.reflection.trivial;

public class TrivialReflection {
    public static void main(String[] args) {
        Class<Class> clazz1 = Class.class;
        Class<Object> clazz2 = Object.class;
        Class<Examinee> clazz3 = Examinee.class;
        Class<ExamineeInterface> clazz4 = ExamineeInterface.class;
        int modifiers1 = clazz1.getModifiers();
        int modifiers2 = clazz2.getModifiers();
        int modifiers3 = clazz3.getModifiers();
        int modifiers4 = clazz4.getModifiers();

        int result = modifiers1 + modifiers2 + modifiers3 + modifiers4;
        System.out.println(result);
    }
}

abstract class Examinee {
}

interface ExamineeInterface {
}
