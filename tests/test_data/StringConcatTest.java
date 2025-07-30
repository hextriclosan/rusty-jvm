package samples.javacore.strings.concat.advanced;

public class StringConcatTest {

    public static void main(String[] args) {
        testBasicConcat();
        testConcatWithPrimitives();
        testConcatInLoop();
        testConcatNulls();
        testConcatWithObjects();
        testConcatWithFinals();
        testConcatWithMultiplePlus();
        testCompileTimeConcat();
    }

    // 1. Basic String + String
    private static void testBasicConcat() {
        String result = "Hello, " + "World!";
        System.out.println("Compile-time constant folding: " + result);
    }

    // 2. String + int, double, boolean, char
    private static void testConcatWithPrimitives() {
        int x = 42;
        double d = 3.14;
        boolean b = true;
        char c = '☺';

        String result = "Value: " + x + ", Pi: " + d + ", Flag: " + b + ", Letter: " + c;
        System.out.println("Concatenation with primitives: " + result);
    }

    // 3. Concatenation inside loop (not efficient, for demonstration)
    private static void testConcatInLoop() {
        String result = "";
        for (int i = 0; i < 5; i++) {
            result += i;
        }
        System.out.println("Concatenation in loop: " + result);
    }

    // 4. Concat involving nulls
    private static void testConcatNulls() {
        String s = null;
        String result = s + " test";
        System.out.println("Concatenation with null: " + result);
    }

    // 5. Concat with object that overrides toString
    private static void testConcatWithObjects() {
        Object obj = new Object() {
            @Override
            public String toString() {
                return "CustomObject";
            }
        };
        String result = "Result: " + obj;
        System.out.println("Concatenation with custom object: " + result);
    }

    // 6. Concat with final variables (might be optimized at compile time)
    private static void testConcatWithFinals() {
        final String a = "foo";
        final String b = "bar";
        String result = a + b;
        System.out.println("Concatenation with final variables: " + result);
    }

    // 7. Long chain of +
    private static void testConcatWithMultiplePlus() {
        String result = "a" + "b" + "c" + "d" + "e";
        System.out.println("Concatenation with multiple plus: " + result);
    }

    // 8. Compile-time constant folding
    private static void testCompileTimeConcat() {
        String a = "hello";
        String b = "world";
        String concat1 = "hello" + "world";         // compile-time folded
        String concat2 = a + b;                     // runtime
        String interned = concat2.intern();

        System.out.println("Compile-time constant folding passed: " + (concat1 == interned));
    }
}
