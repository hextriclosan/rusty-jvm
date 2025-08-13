package samples.reflection.getdeclaredmethod;

import java.lang.reflect.Method;

public class GetDeclaredMethodExample {

    public static void main(String[] args) throws Exception {
        // Static method invocation using reflection
        Method ofMethod = Person.class.getDeclaredMethod("of", String.class, int.class, String[].class);
        System.out.println("ofMethod found: " + ofMethod);
        Person person = (Person) ofMethod.invoke(null, "Alice", 25, new String[]{"Reading", "Hiking"});
        System.out.println("Person created: " + person);

        // Try a method without parameters
        Method ageMethod = Person.class.getDeclaredMethod("getAgeAsString");
        System.out.println("ageMethod found: " + ageMethod);
        ageMethod.setAccessible(true);
        System.out.println("Age as String: " + ageMethod.invoke(person));

        // Get a declared method (private) from Person class
        Method formatNameMethod = Person.class.getDeclaredMethod("formatName", String.class);
        System.out.println("formatNameMethod found: " + formatNameMethod);
        formatNameMethod.setAccessible(true);
        String result = (String) formatNameMethod.invoke(person, "Dr.");
        System.out.println("Result: " + result);

        // Set age using a setter method
        Method setAgeMethod = Person.class.getDeclaredMethod("setAge", int.class);
        System.out.println("setAgeMethod found: " + setAgeMethod);
        setAgeMethod.invoke(person, 30);

        // Set hobbies using a setter method
        Method setHobbiesMethod = Person.class.getDeclaredMethod("setHobbies", String[].class);
        System.out.println("setHobbiesMethod found: " + setHobbiesMethod);
        setHobbiesMethod.invoke(person, (Object) new String[]{"Swimming", "Cycling"});

        System.out.println("Updated Person: " + person);
    }
}

class Person {
    private String name;
    private int age;
    private String[] hobbies;

    public static Person of(String name, int age, String[] hobbies) {
        return new Person(name, age, hobbies);
    }

    private Person(String name, int age, String[] hobbies) {
        this.name = name;
        this.age = age;
        this.hobbies = hobbies;
    }

    private String formatName(String prefix) {
        return prefix + " " + name;
    }

    private String getAgeAsString() {
        return Integer.toString(age);
    }

    public void setHobbies(String[] hobbies) {
        this.hobbies = hobbies;
    }

    public void setAge(int age) {
        this.age = age;
    }

    @Override
    public String toString() {
        return "Person{name='" + name + "', age=" + age + ", hobbies=" + String.join(", ", hobbies) + "}";
    }
}
