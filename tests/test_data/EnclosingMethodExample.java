package samples.reflection.trivial.enclosingmethod;

import java.lang.reflect.Method;

public class EnclosingMethodExample {

    // Static nested class (not enclosed in a method)
    static class StaticNestedClass {
    }

    // Non-static inner class (not enclosed in a method)
    class NonStaticInnerClass {
    }

    public static void main(String[] args) {
        new EnclosingMethodExample().testEnclosingMethods();
    }

    public void testEnclosingMethods() {
        // 1. Top-level class
        Class<?> topLevelClass = EnclosingMethodExample.class;
        System.out.print("Top-level class enclosing method: ");
        System.out.println(topLevelClass.getEnclosingMethod() != null ? topLevelClass.getEnclosingMethod().getName() : "null");

        // 2. Static nested class
        Class<?> staticNestedClass = StaticNestedClass.class;
        System.out.print("StaticNestedClass enclosing method: ");
        System.out.println(staticNestedClass.getEnclosingMethod() != null ? staticNestedClass.getEnclosingMethod().getName() : "null");

        // 3. Non-static inner class
        Class<?> nonStaticInnerClass = NonStaticInnerClass.class;
        System.out.print("NonStaticInnerClass enclosing method: ");
        System.out.println(nonStaticInnerClass.getEnclosingMethod() != null ? nonStaticInnerClass.getEnclosingMethod().getName() : "null");

        // 4. Local class within a method
        class LocalClass {
        }
        Method localClassEnclosingMethod = LocalClass.class.getEnclosingMethod();
        System.out.print("LocalClass enclosing method: ");
        System.out.println(localClassEnclosingMethod != null ? localClassEnclosingMethod.getName() : "null");

        // 5. Anonymous class within a method
        Object anonymousClass = new Object() {};
        Method anonymousClassEnclosingMethod = anonymousClass.getClass().getEnclosingMethod();
        System.out.print("AnonymousClass enclosing method: ");
        System.out.println(anonymousClassEnclosingMethod != null ? anonymousClassEnclosingMethod.getName() : "null");

        // 6. Anonymous class within a constructor
        Object anonymousInConstructor = new Object() {
            {
                System.out.println("Inside anonymous constructor initializer.");
            }
        };
        Method anonymousInConstructorEnclosingMethod = anonymousInConstructor.getClass().getEnclosingMethod();
        System.out.print("AnonymousClass in constructor enclosing method: ");
        System.out.println(anonymousInConstructorEnclosingMethod != null ? anonymousInConstructorEnclosingMethod.getName() : "null");

        // 7. Local class within a constructor
        class LocalClassInConstructor {
        }
        Method localClassInConstructorEnclosingMethod = LocalClassInConstructor.class.getEnclosingMethod();
        System.out.print("LocalClass in constructor enclosing method: ");
        System.out.println(localClassInConstructorEnclosingMethod != null ? localClassInConstructorEnclosingMethod.getName() : "null");

        // 8. Anonymous class within a static block
//         Object anonymousInStaticBlock = AnonymousStaticBlock.getAnonymous(); // todo: fix throwing InternalError
//         Method anonymousInStaticBlockEnclosingMethod = anonymousInStaticBlock.getClass().getEnclosingMethod();
//         System.out.print("AnonymousClass in static block enclosing method: ");
//         System.out.println(anonymousInStaticBlockEnclosingMethod != null ? anonymousInStaticBlockEnclosingMethod.getName() : "null");
    }

    // Utility class for static block example
    static class AnonymousStaticBlock {
        static Object getAnonymous() {
            return new Object() {};
        }
    }
}
