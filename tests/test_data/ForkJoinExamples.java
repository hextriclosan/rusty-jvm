package samples.concurrency.forkjoin;

import java.util.Arrays;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ForkJoinPool;
import java.util.concurrent.ForkJoinTask;
import java.util.concurrent.RecursiveAction;
import java.util.concurrent.RecursiveTask;
import java.util.concurrent.TimeUnit;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.LongStream;

/**
 * Exercises the ForkJoin framework and parallel streams. Every result is deterministic regardless of
 * how work is scheduled across pool workers, so the expected output is stable.
 *
 * The stress loop and nested-parallelism cases deliberately spin up many pool workers that park
 * and must be woken to pick up work: if worker wake-up signalling regresses the pool deadlocks and
 * this program hangs instead of finishing, which is exactly what the test guards against.
 */
public class ForkJoinExamples {
    public static void main(String[] args) throws Exception {
        recursiveTaskSum();
        recursiveActionDouble();
        customPoolInvoke();
        invokeAllSubtasks();
        parallelStreamReduce();
        parallelStreamCollect();
        nestedParallelism();
        stressLoop();
        System.out.println("DONE");
    }

    /** Divide-and-conquer sum via {@link RecursiveTask#fork()}/{@link RecursiveTask#join()}. */
    static final class SumTask extends RecursiveTask<Long> {
        private final long[] a;
        private final int lo;
        private final int hi;

        SumTask(long[] a, int lo, int hi) {
            this.a = a;
            this.lo = lo;
            this.hi = hi;
        }

        @Override
        protected Long compute() {
            if (hi - lo <= 500) {
                long s = 0;
                for (int i = lo; i < hi; i++) {
                    s += a[i];
                }
                return s;
            }
            int mid = (lo + hi) >>> 1;
            SumTask left = new SumTask(a, lo, mid);
            left.fork();
            long right = new SumTask(a, mid, hi).compute();
            return left.join() + right;
        }
    }

    static void recursiveTaskSum() {
        long[] a = new long[10_000];
        for (int i = 0; i < a.length; i++) {
            a[i] = i + 1;
        }
        long total = ForkJoinPool.commonPool().invoke(new SumTask(a, 0, a.length));
        System.out.println("RecursiveTask sum 1..10000 = " + total);
    }

    /** In-place transform via {@link RecursiveAction} and {@link ForkJoinTask#invokeAll}. */
    static final class DoubleAction extends RecursiveAction {
        private final int[] a;
        private final int lo;
        private final int hi;

        DoubleAction(int[] a, int lo, int hi) {
            this.a = a;
            this.lo = lo;
            this.hi = hi;
        }

        @Override
        protected void compute() {
            if (hi - lo <= 500) {
                for (int i = lo; i < hi; i++) {
                    a[i] *= 2;
                }
                return;
            }
            int mid = (lo + hi) >>> 1;
            invokeAll(new DoubleAction(a, lo, mid), new DoubleAction(a, mid, hi));
        }
    }

    static void recursiveActionDouble() {
        int[] a = new int[8_000];
        Arrays.fill(a, 1);
        ForkJoinPool.commonPool().invoke(new DoubleAction(a, 0, a.length));
        long sum = 0;
        for (int v : a) {
            sum += v;
        }
        System.out.println("RecursiveAction doubled sum = " + sum);
    }

    static void customPoolInvoke() throws Exception {
        ForkJoinPool pool = new ForkJoinPool(3);
        try {
            long[] a = new long[5_000];
            Arrays.fill(a, 1L);
            long r = pool.invoke(new SumTask(a, 0, a.length));
            System.out.println("Custom pool sum = " + r + ", parallelism = " + pool.getParallelism());
        } finally {
            pool.shutdown();
            pool.awaitTermination(5, TimeUnit.SECONDS);
        }
    }

    static void invokeAllSubtasks() {
        long[] a = new long[6_000];
        Arrays.fill(a, 2L);
        List<SumTask> tasks =
                List.of(
                        new SumTask(a, 0, 2_000),
                        new SumTask(a, 2_000, 4_000),
                        new SumTask(a, 4_000, 6_000));
        ForkJoinTask.invokeAll(tasks);
        long total = 0;
        for (SumTask t : tasks) {
            total += t.join();
        }
        System.out.println("invokeAll sum = " + total);
    }

    static void parallelStreamReduce() {
        long sum = LongStream.rangeClosed(1, 100_000).parallel().sum();
        System.out.println("parallel LongStream sum = " + sum);
        int product = IntStream.rangeClosed(1, 12).boxed().parallel().reduce(1, (x, y) -> x * y);
        System.out.println("parallel product 1..12 = " + product);
    }

    static void parallelStreamCollect() {
        List<Integer> evens =
                IntStream.range(0, 20_000).parallel().filter(n -> n % 2 == 0).boxed()
                        .collect(Collectors.toList());
        System.out.println(
                "parallel evens: count = "
                        + evens.size()
                        + ", first = "
                        + evens.get(0)
                        + ", last = "
                        + evens.get(evens.size() - 1));
        Map<Boolean, Long> partitioned =
                IntStream.range(0, 10_000).parallel().boxed()
                        .collect(Collectors.partitioningBy(n -> n % 3 == 0, Collectors.counting()));
        System.out.println(
                "parallel partition %3: divisible = "
                        + partitioned.get(true)
                        + ", rest = "
                        + partitioned.get(false));
    }

    /** Nested parallelism: each element of an outer parallel stream runs its own parallel reduce. */
    static void nestedParallelism() {
        long total =
                IntStream.rangeClosed(1, 50).parallel()
                        .mapToLong(k -> LongStream.rangeClosed(1, 1_000).parallel().sum())
                        .sum();
        System.out.println("nested parallel total = " + total);
    }

    /** Repeats a parallel reduction many times to stress pool worker wake-up under contention. */
    static void stressLoop() {
        long expected = LongStream.rangeClosed(1, 50_000).sum();
        int iterations = 40;
        boolean consistent = true;
        for (int i = 0; i < iterations; i++) {
            long s = LongStream.rangeClosed(1, 50_000).parallel().map(x -> x).sum();
            if (s != expected) {
                consistent = false;
            }
        }
        System.out.println("stress loop " + iterations + " iterations consistent = " + consistent);
    }
}
