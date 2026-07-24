package samples.concurrency.atomics;

import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.AtomicLong;
import java.util.concurrent.atomic.AtomicReference;

/**
 * Directly exercises the {@code compareAndSet} / {@code compareAndExchange} atomics that back the
 * ForkJoin pool's coordination words. {@code AtomicInteger}/{@code AtomicLong}/{@code AtomicReference}
 * route these through {@code Unsafe.compareAndExchangeInt/Long/Reference} (and, on some paths, the
 * corresponding {@code VarHandle} CAS), so this is a focused regression guard for those primitives.
 */
public class AtomicCasExamples {
    public static void main(String[] args) {
        AtomicInteger i = new AtomicInteger(10);
        // compareAndExchange returns the witness (prior) value.
        System.out.println("int witness on success = " + i.compareAndExchange(10, 20));
        System.out.println("int witness on failure = " + i.compareAndExchange(999, 30));
        System.out.println("int value = " + i.get());
        System.out.println("int compareAndSet fail = " + i.compareAndSet(999, 40));
        System.out.println("int compareAndSet ok = " + i.compareAndSet(20, 40));
        System.out.println("int getAndAdd = " + i.getAndAdd(2) + " -> " + i.get());

        AtomicLong l = new AtomicLong(5_000_000_000L);
        System.out.println("long witness on success = " + l.compareAndExchange(5_000_000_000L, 6_000_000_000L));
        System.out.println("long compareAndSet ok = " + l.compareAndSet(6_000_000_000L, 7_000_000_000L));
        System.out.println("long value = " + l.get());

        AtomicReference<String> r = new AtomicReference<>("a");
        System.out.println("ref witness on success = " + r.compareAndExchange("a", "b"));
        System.out.println("ref witness on failure = " + r.compareAndExchange("zzz", "c"));
        System.out.println("ref compareAndSet ok = " + r.compareAndSet("b", "c"));
        System.out.println("ref value = " + r.get());
        System.out.println("ref updateAndGet = " + r.updateAndGet(s -> s + "!"));
    }
}
