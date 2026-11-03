package samples.casts;

import java.util.ArrayList;
import java.util.List;

public class ClassCastCheck {
    interface Animal {
    }

    static class Dog implements Animal {
        @Override
        public String toString() {
            return "dog";
        }
    }

    static class Cat implements Animal {
    }

    interface Cast {
        Object run();
    }

    public static void main(String[] args) {
        Object integer = 42;
        Object string = "value";
        Object dog = new Dog();
        Object strings = new String[] {"a"};
        Object ints = new int[] {1};
        Object dogs = new Dog[] {new Dog()};
        Object nothing = null;

        run("same class", () -> (String) string);
        run("superclass", () -> (Number) integer);
        run("interface", () -> (Animal) dog);
        run("null", () -> (Dog) nothing);
        run("covariant array", () -> (Animal[]) dogs);
        run("array as Object[]", () -> (Object[]) strings);
        run("array as Cloneable", () -> (Cloneable) ints);

        run("both in java.base", () -> (String) integer);
        run("both application classes", () -> (Cat) dog);
        run("java.base to application", () -> (Dog) string);
        run("application to java.base", () -> (Runnable) dog);
        run("object arrays", () -> (Integer[]) strings);
        run("primitive arrays", () -> (long[]) ints);
        run("application array to java.base array", () -> (String[]) dogs);
        run("object to array", () -> (int[]) string);
        run("array to class", () -> (String) ints);
        run("primitive array to Object[]", () -> (Object[]) ints);

        List raw = new ArrayList();
        raw.add(42);
        List<String> typed = raw;
        run("heap pollution", () -> typed.get(0).length());

        System.out.println("caught in the same frame: " + castInTry(integer));
    }

    private static String castInTry(Object value) {
        try {
            return (String) value;
        } catch (ClassCastException exception) {
            return "caught";
        }
    }

    private static void run(String label, Cast cast) {
        try {
            Object result = cast.run();
            System.out.println(label + " ok " + (result instanceof Object[] array ? java.util.Arrays.toString(array) : result instanceof int[] ? "int[]" : result));
        } catch (ClassCastException exception) {
            System.out.println(label + " -> " + exception.getMessage());
        }
    }
}
