package samples.javacore.invokedynamic.lambda;


import java.math.BigInteger;
import java.util.ArrayList;
import java.util.List;
import java.util.function.BiFunction;
import java.util.function.BinaryOperator;
import java.util.function.Consumer;
import java.util.function.Function;
import java.util.function.IntUnaryOperator;
import java.util.function.Predicate;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class LambdaExample {
    public static void main(String[] args) {
        // Basic method reference
        Function<String, Integer> lengthFunc = String::length;
        System.out.println("Length: " + lengthFunc.apply("invokedynamic"));

        // Lambda expression
        Function<String, String> concatFunc = s -> s.concat(" dynamic");
        System.out.println(concatFunc.apply("invoke"));

        // Method reference to instance method
        Consumer<String> printer = System.out::println;
        printer.accept("Hello from method reference!");

        // Lambdas with functional interface
        List<MyOp> ops = List.of((a, b) -> a + b, (a, b) -> a - b, (a, b) -> a * b, (a, b) -> a / b);
        ops.forEach(op -> {
            int result = operation(op::operate, 10, 5);
            System.out.println("Result: " + result);
        });

        // Stream + method reference
        String joined = IntStream.range(1, 5)
                .boxed()
                .map(Object::toString)
                .collect(Collectors.joining(", "));
        System.out.println("Joined numbers: " + joined);

        // Method reference with many types
        int i = 1;
        long l = 2L;
        double d = 3.0;
        boolean b = true;
        short s = 4;
        float f = 5.0f;
        String str = "test";
        BigInteger bi = BigInteger.TEN;
        AllTypes concatAll = LambdaExample::allTypesConcat;
        System.out.println(concatAll.concat(i, l, d, b, s, f, str, bi));

        // Capturing variables
        Runnable captured = () -> System.out.println("Captured: " + i + ", " + l + ", " + d + ", " + b + ", " + s + ", " + f + ", " + str + ", " + bi);
        captured.run();

        // Higher-order function returning lambda
        Function<String, Predicate<String>> makeStartsWith = prefix -> input -> input.startsWith(prefix);
        Predicate<String> startsWithTest = makeStartsWith.apply("dyn");
        System.out.println("startsWith 'dyn'? " + startsWithTest.test("dynamic"));

        // Generic lambda usage
        BinaryOperator<BigInteger> bigIntAdder = BigInteger::add;
        System.out.println("BigInteger add: " + bigIntAdder.apply(BigInteger.valueOf(100), BigInteger.valueOf(23)));

        // Sorting with lambda and method reference
        List<String> words = new ArrayList<>(List.of("banana", "apple", "cherry"));
        words.sort(String::compareToIgnoreCase);
        words.forEach(w -> System.out.println("Sorted: " + w));

        // Using BiFunction + partial application (currying-like)
        BiFunction<Integer, Integer, Integer> multiply = (x, y) -> x * y;
        Function<Integer, Integer> times10 = y -> multiply.apply(10, y);
        System.out.println("10 * 7 = " + times10.apply(7));

        // Lambda that returns a lambda
        Function<Integer, IntUnaryOperator> multiplierFactory = factor -> x -> x * factor;
        IntUnaryOperator triple = multiplierFactory.apply(3);
        System.out.println("Triple 6: " + triple.applyAsInt(6));
    }

    private static int operation(BiFunction<Integer, Integer, Integer> func, int a, int b) {
        return func.apply(a, b);
    }

    private static String allTypesConcat(int i, long l, double d, boolean b, short s, float f, String str, BigInteger bi) {
        return i + " " + l + " " + d + " " + b + " " + s + " " + f + " " + str + " " + bi;
    }
}

@FunctionalInterface
interface MyOp {
    int operate(int a, int b);
}

@FunctionalInterface
interface AllTypes {
    String concat(int i, long l, double d, boolean b, short s, float f, String str, BigInteger bi);
}
