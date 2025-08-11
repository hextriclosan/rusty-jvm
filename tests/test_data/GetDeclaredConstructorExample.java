package samples.reflection.getdeclaredconstructorexample;

import java.lang.reflect.Constructor;

public class GetDeclaredConstructorExample {
    public static void main(String[] args) throws Exception {
        Constructor<Person> ctor1 = Person.class.getDeclaredConstructor();
        System.out.println("Person() found: " + ctor1);
        ctor1.setAccessible(true);
        Person p1 = ctor1.newInstance();
        System.out.println("Created: " + p1);

        Constructor<Person> ctor2 = Person.class.getDeclaredConstructor(String.class);
        System.out.println("Person(String name, int age) found: " + ctor2);
        ctor2.setAccessible(true);
        Person p2 = ctor2.newInstance("Max");
        System.out.println("Created: " + p2);

//         Constructor<Person> ctor3 = Person.class.getDeclaredConstructor(String.class, int.class); // fixme Execution Error: error opening class file java/lang/invoke/BoundMethodHandle$Species_LI
//         System.out.println("Person(String name, int age) found: " + ctor3);
//         ctor3.setAccessible(true);
//         Person p3 = ctor3.newInstance("Alice", 30);
//         System.out.println("Created: " + p3);

        Constructor<Person> ctor4 = Person.class.getDeclaredConstructor(String.class, String[].class);
        System.out.println("Person(String name, String[] hobbies) found: " + ctor4);
        ctor4.setAccessible(true);
        Person p4 = ctor4.newInstance("Deborah", new String[]{"Cycling", "Cooking"});
        System.out.println("Created: " + p4);

//         Constructor<Person> ctor5 = Person.class.getDeclaredConstructor(String.class, int.class, String[].class); // fixme Execution Error: error opening class file java/lang/invoke/BoundMethodHandle$Species_LI
//         System.out.println("Person(String name, int age, String[] hobbies) found: " + ctor5);
//         ctor5.setAccessible(true);
//         Person p5 = ctor4.newInstance("Michael", 42, new String[]{"Reading", "Hiking"});
//         System.out.println("Created: " + p5);
    }
}

class Person {
    private String name;
    private int age;
    private String[] hobbies;

    private Person() {
        this("John Doe", 0, new String[]{});
    }

    private Person(String name) {
        this(name, 0, new String[]{});
    }

    private Person(String name, int age) {
        this(name, age, new String[]{});
    }

    private Person(String name, String[] hobbies) {
        this(name, 0, hobbies);
    }

    private Person(String name, int age, String[] hobbies) {
        this.name = name;
        this.age = age;
        this.hobbies = hobbies;
    }

    @Override
    public String toString() {
        return name + " (" + age + ")" + " - Hobbies: [" + String.join(", ", hobbies) + "]";
    }
}
