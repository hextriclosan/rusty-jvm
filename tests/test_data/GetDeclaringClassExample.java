package samples.reflection.trivial.classgetdeclaringclassexample;

public class GetDeclaringClassExample {
    public static class TopLevel {
    }

    public static class SimpleNested {
        public class Inner {
        }

        public static class StaticNested {
        }
    }

    public static void main(String[] args) {
        print(double.class);

        print(String[].class);

        print(GetDeclaringClassExample.class);

        print(TopLevel.class);

        print(SimpleNested.Inner.class);

        print(SimpleNested.StaticNested.class);

        Runnable anonymous = new Runnable() {
            @Override
            public void run() {
            }
        };
        print(anonymous.getClass());

        class LocalClass {
        }
        print(LocalClass.class);

        testStaticMethodInner();

        DeepNesting outer = new DeepNesting();
        DeepNesting.Inner inner = outer.new Inner();
        DeepNesting.Inner.DeepInner deepInner = inner.new DeepInner();
        print(DeepNesting.Inner.class);
        print(DeepNesting.Inner.DeepInner.class);
        print(deepInner.getClass());

//         Runnable lambda = () -> {
//         };
//         print(lambda.getClass()); // Execution Error: Unknown reference opcode: 186
    }

    private static void testStaticMethodInner() {
        class StaticMethodInner {
        }
        print(StaticMethodInner.class);
    }

    static class DeepNesting {
        public class Inner {
            public class DeepInner {
            }
        }
    }

    private static void print(Class<?> clazz) {
        System.out.println(clazz.getName() + ": " + clazz.getDeclaringClass());
    }

}
