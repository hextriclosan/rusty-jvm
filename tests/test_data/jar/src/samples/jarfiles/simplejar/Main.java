package samples.jarfiles.simplejar;

import java.util.Iterator;
import java.util.List;
import io.github.hextriclosan.algorithm.iterators.NextPermutationIterator;

public class Main {
    public static void main(String[] args) {
        Iterator<List<Character>> iterator = new NextPermutationIterator<>(List.of('A', 'B', 'B', 'C'));
        iterator.forEachRemaining(System.out::println);
    }
}
