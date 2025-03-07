package samples.reflection.trivial.classisinstanceexample;

import java.util.AbstractMap;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;

public class ClassIsInstanceExample {

    public static void main(String[] args) {
        testInstance(null, Integer.class);
        testInstance(42, Integer.class);
        testInstance(42, Number.class);
        testInstance(3.14, Double.class);
        testInstance(3.14, Number.class);
        testInstance(3.14, Float.class);
        testInstance("Hello", String.class);
        testInstance("Hello", Object.class);

        testInstance(new int[]{1}, Object.class);
        testInstance(new int[]{1, 2}, Object[].class);
        testInstance(new int[]{1, 2, 3}, int[].class);
        testInstance(new Integer[]{10, 20, 30}, Object.class);
        testInstance(new String[]{"A", "B"}, Object[].class);
        testInstance(new Cat[]{new Cat(), new Cat()}, Cat[].class);
        testInstance(new Cat[]{new Cat(), new Cat()}, Animal[].class);
        testInstance(new Cat[]{new Cat(), new Cat()}, Mammal[].class);
        testInstance(new Cat[]{new Cat(), new Cat()}, AbstractMammal[].class);
        testInstance(new Animal[]{new Cat(), new Dog()}, Cat[].class);
        testInstance(new Animal[]{new Cat(), new Dog()}, Animal[].class);
        testInstance(new Animal[]{new Cat(), new Dog()}, Mammal[].class);
        testInstance(new Animal[]{new Cat(), new Dog()}, AbstractMammal[].class);
        testInstance(new ArrayList[1], List[].class);

        testInstance(new ArrayList<>(), List.class);
        testInstance(new HashMap<>(), Map.class);
        testInstance(new HashMap<>(), AbstractMap.class);
        testInstance(new HashSet<>(), Collection.class);

        testInstance(new Cat(), Animal.class);
        testInstance(new Bird(), Animal.class);
        testInstance(new Bird(), Mammal.class);
        Animal anonymous = new AbstractMammal() {
            @Override
            public String say() {
                return "Anonymous mammal says hi!";
            }
        };
        testInstance(anonymous, Animal.class);
    }

    private static void testInstance(Object obj, Class<?> clazz) {
        String objString = formatArray(obj);
        System.out.printf("%s is instance of %s.class: %b%n", objString, clazz.getSimpleName(), clazz.isInstance(obj));
    }

    private static String formatArray(Object obj) {
        if (obj instanceof int[]) return Arrays.toString((int[]) obj);
        if (obj instanceof double[]) return Arrays.toString((double[]) obj);
        if (obj instanceof long[]) return Arrays.toString((long[]) obj);
        if (obj instanceof char[]) return Arrays.toString((char[]) obj);
        if (obj instanceof byte[]) return Arrays.toString((byte[]) obj);
        if (obj instanceof short[]) return Arrays.toString((short[]) obj);
        if (obj instanceof float[]) return Arrays.toString((float[]) obj);
        if (obj instanceof boolean[]) return Arrays.toString((boolean[]) obj);
        if (obj instanceof Object[]) return Arrays.deepToString((Object[]) obj);
        return String.valueOf(obj);
    }
}

interface Animal {
    String say();
}

interface Mammal extends Animal {
}

abstract class AbstractMammal implements Mammal {
    @Override
    public String toString() {
        return say();
    }
}

class Dog extends AbstractMammal {
    @Override
    public String say() {
        return "Woof!";
    }
}

class Cat extends AbstractMammal {
    @Override
    public String say() {
        return "Meow!";
    }
}

class Bird implements Animal {
    @Override
    public String say() {
        return "Chirp!";
    }

    @Override
    public String toString() {
        return say();
    }
}
