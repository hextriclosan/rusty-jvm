package samples.javacore.invokedynamic.lambda;

import java.util.function.BiFunction;
import java.util.List;
import java.util.function.Consumer;
import java.util.function.Function;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class LambdaExample {
    public static void main(String[] args) {
        Function<String, Integer> lengthFunc = String::length;
        System.out.println(lengthFunc.apply("invokedynamic"));

        Function<String, String> concatFunc = s -> s.concat(" dynamic");
        System.out.println(concatFunc.apply("invoke"));

        Consumer<String> printer = System.out::println;
        printer.accept("Hello from method reference!");

        List<MyOp> ops = List.of((a, b) -> a + b, (a, b) -> a - b, (a, b) -> a * b, (a, b) -> a / b);
        ops.forEach(op -> {
            int result = operation(op::operate, 10, 5);
            System.out.println("Result: " + result);
        });

        String joined = IntStream.range(1, 5)
                .boxed()
                .map(Object::toString)
                .collect(Collectors.joining(", "));
        System.out.println("Joined numbers: " + joined);
    }

    private static int operation(BiFunction<Integer, Integer, Integer> func, int a, int b) {
        return func.apply(a, b);
    }
}

@FunctionalInterface
interface MyOp {
    int operate(int a, int b);
}
