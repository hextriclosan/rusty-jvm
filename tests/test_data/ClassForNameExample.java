package samples.reflection.trivial.forname;

public class ClassForNameExample {
    public static void main(String[] args) throws ClassNotFoundException {
        Class<?> firstClass = Class.forName("samples.reflection.trivial.forname.First");
        System.out.println(firstClass);

        Class<?> secondClass = Class.forName("samples.reflection.trivial.forname.Second", false, ClassForNameExample.class.getClassLoader());
        System.out.println(secondClass);
    }
}

class First {
    static {
        System.out.println("First static block");
    }
}

class Second {
    static {
        System.out.println("Second static block");
    }
}
