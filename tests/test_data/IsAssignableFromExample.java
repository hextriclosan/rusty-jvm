package samples.reflection.trivial.isassignablefromexample;

import java.util.AbstractMap;
import java.util.ArrayList;
import java.util.Collection;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;

public class IsAssignableFromExample {

    public static void main(String[] args) throws ClassNotFoundException {
        testClasses(int.class, int.class);
        testClasses(Integer.class, int.class);
        testClasses(Number.class, Integer.class);
        testClasses(double.class, int.class);
        testClasses(double.class, float.class);
        testClasses(float.class, double.class);
        testClasses(double.class, Double.class);
        testClasses(Double.class, double.class);

        testClasses(Cat.class, Animal.class);
        testClasses(Bird.class, Animal.class);
        testClasses(Bird.class, Mammal.class);
        testClasses(Animal.class, Cat.class);
        testClasses(Animal.class, Bird.class);
        testClasses(Mammal.class, Bird.class);

        testClasses(Object.class, int[].class);
        testClasses(Object.class, Integer[].class);
        testClasses(Object[].class, String[].class);
        testClasses(Animal[].class, Cat[].class);
        testClasses(Cat[].class, Animal[].class);
        testClasses(Object[].class, int[].class);
        testClasses(List[].class, ArrayList[].class);

        testClasses(Object.class, Object.class);
        testClasses(Object.class, String.class);
        testClasses(String.class, Object.class);
        testClasses(String.class, String.class);
        testClasses(Integer.class, Integer.class);
        testClasses(Integer.class, Number.class);
        testClasses(Number.class, Integer.class);
        testClasses(Number.class, Number.class);

        testClasses(List.class, ArrayList.class);
        testClasses(ArrayList.class, List.class);

        testClasses(Map.class, HashMap.class);
        testClasses(AbstractMap.class, HashMap.class);
        testClasses(HashMap.class, AbstractMap.class);

        testClasses(Collection.class, HashSet.class);
        testClasses(HashSet.class, Collection.class);

        Class<?> map = Class.forName("java.util.Map");
        Class<?> hashMap = Class.forName("java.util.HashMap");
        testClasses(map, hashMap);

        Class<?> animal = Class.forName("samples.reflection.trivial.isassignablefromexample.Animal");
        Class<?> dog = Class.forName("samples.reflection.trivial.isassignablefromexample.Dog");
        testClasses(animal, dog);

        AbstractMammal anonymous = new AbstractMammal() {
            @Override
            public String say() {
                return "Anonymous mammal says hi!";
            }
        };
        testClasses(Animal.class, anonymous.getClass());
    }

    private static void testClasses(Class<?> assignable, Class<?> assignee) {
        System.out.printf("%s is assignable from %s: %b%n", assignable.getSimpleName(), assignee.getSimpleName(), assignable.isAssignableFrom(assignee));
    }

}

interface Animal {
    String say();
}

interface Mammal extends Animal {
}

abstract class AbstractMammal implements Mammal {
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
}
