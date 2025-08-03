package samples.reflection.trivial.classgetnesthostexample;

public class ClassGetNestHostExample {
    public static void main(String[] args) {
        print("ClassGetNestHostExample", ClassGetNestHostExample.class);
        print("NestedClass", NestedClass.class);
        print("InnerClass", InnerClass.class);

        class LocalClass {
        }
        print("LocalClass", LocalClass.class);

        print("Runnable", new Runnable() {
            @Override
            public void run() {}
        }.getClass());

        print("InnerInterface", new InnerInterface() {
        }.getClass());

        Runnable lambda1 = () -> {};
        print("A lambda 1", lambda1.getClass());
        Runnable lambda2 = () -> {};
        print("A lambda 2", lambda2.getClass());

        print("String", String.class);
        print("int", int.class);
        print("ClassGetNestHostExample[]", ClassGetNestHostExample[].class);
    }

    static class NestedClass {
    }

    class InnerClass {
    }

    interface InnerInterface {
    }

    private static void print(String description, Class<?> clazz) {
        Class<?> nestHost = clazz.getNestHost();
        System.out.printf("%s (%s): %s%n", description, clazz, nestHost);
    }
}
