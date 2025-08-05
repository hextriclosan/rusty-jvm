package samples.javacore.streams.streamexamples;

import java.math.BigInteger;
import java.util.*;
import java.util.function.Function;
import java.util.stream.*;

public class StreamExamples {
    public static void main(String[] args) {
        basicStream();
        //mapExample(); // Execution Error: Unsupported yet reference kind for invokedynamic: REF_newInvokeSpecial
        filterExample();
        reduceExample();
        collectExample();
        //groupByExample(); // Execution Error: Unsupported yet reference kind for invokedynamic: REF_newInvokeSpecial
        flatMapExample();
        infiniteStreamExample();
        //customCollectorExample(); // Execution Error: Unsupported yet reference kind for invokedynamic: REF_newInvokeSpecial
        //parallelStreamExample(); // Native Call Error: Native method java/lang/System:nanoTime:()J not found
    }

    static void basicStream() {
        Stream.of("a", "b", "c").forEach(System.out::println);
    }

    static void mapExample() {
        List<String> names = List.of("Alice", "Bob", "Charlie");
        List<Integer> lengths = names.stream()
                                     .map(String::length)
                                     .collect(Collectors.toList());
        System.out.println("Lengths: " + lengths);
    }

    static void filterExample() {
        List<Integer> numbers = IntStream.range(0, 10).boxed().toList();
        List<Integer> even = numbers.stream()
                                    .filter(n -> n % 2 == 0)
                                    .toList();
        System.out.println("Even numbers: " + even);
    }

    static void reduceExample() {
        int sum = IntStream.rangeClosed(1, 5).reduce(0, Integer::sum);
        System.out.println("Sum 1 to 5: " + sum);
    }

    static void collectExample() {
        String result = Stream.of("a", "b", "c")
                              .collect(Collectors.joining("-"));
        System.out.println("Joined: " + result);
    }

    static void groupByExample() {
        List<String> words = List.of("apple", "banana", "apricot", "blueberry");
        Map<Character, List<String>> grouped = words.stream()
                .collect(Collectors.groupingBy(word -> word.charAt(0)));
        System.out.println("Grouped: " + grouped);
    }

    static void flatMapExample() {
        List<List<String>> nested = List.of(
                List.of("a", "b"),
                List.of("c", "d"),
                List.of("e")
        );
        List<String> flattened = nested.stream()
                                       .flatMap(List::stream)
                                       .toList();
        System.out.println("Flattened: " + flattened);
    }

    static void infiniteStreamExample() {
        List<Integer> squares = Stream.iterate(1, n -> n + 1)
                                      .map(n -> n * n)
                                      .limit(5)
                                      .toList();
        System.out.println("First 5 squares: " + squares);
    }

    static void customCollectorExample() {
        List<String> words = List.of("apple", "banana", "pear");
        String upperJoined = words.stream().collect(
                Collector.of(
                        StringBuilder::new,
                        (sb, s) -> sb.append(s.toUpperCase()).append(" "),
                        StringBuilder::append,
                        StringBuilder::toString
                )
        );
        System.out.println("Custom collected: " + upperJoined.trim());
    }

    static void parallelStreamExample() {
        List<Integer> numbers = IntStream.range(1, 10).boxed().toList();
        int product = numbers.parallelStream()
                             .reduce(1, (a, b) -> a * b);
        System.out.println("Product (parallel): " + product);
    }
}
