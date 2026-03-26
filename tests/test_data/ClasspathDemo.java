package samples.javacore.classpath.classpathdemo;

import java.util.Iterator;
import java.util.List;
import io.github.hextriclosan.algorithm.iterators.NextPermutationIterator;

public class ClasspathDemo {
    public static void main(String[] args) {
        Iterator<List<Character>> iterator = new NextPermutationIterator<>(List.of('A', 'B', 'B'));
        iterator.forEachRemaining(System.out::println);
    }
}
