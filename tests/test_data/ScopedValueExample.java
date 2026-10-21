package samples.concurrency.scopedvalue;

import java.util.NoSuchElementException;

public class ScopedValueExample {
    private static final ScopedValue<String> USER = ScopedValue.newInstance();
    private static final ScopedValue<Integer> LEVEL = ScopedValue.newInstance();

    public static void main(String[] args) throws Exception {
        System.out.println("bound outside: " + USER.isBound());

        ScopedValue.where(USER, "alice").run(() -> {
            // repeated reads are served from the per-thread scoped value cache
            for (int i = 0; i < 3; i++) {
                System.out.println("user: " + USER.get());
            }
            ScopedValue.where(USER, "bob").where(LEVEL, 2).run(() ->
                    System.out.println("nested: " + USER.get() + " level " + LEVEL.get()));
            System.out.println("restored: " + USER.get() + ", level bound: " + LEVEL.isBound());
        });

        String result = ScopedValue.where(USER, "carol").call(() -> USER.get().toUpperCase());
        System.out.println("call result: " + result);
        System.out.println("orElse outside: " + USER.orElse("nobody"));
        try {
            USER.get();
        } catch (NoSuchElementException e) {
            System.out.println("unbound get: " + e.getClass().getSimpleName());
        }

        // bindings are not inherited by a plain new thread
        ScopedValue.where(USER, "dave").run(() -> {
            Thread thread = new Thread(() -> System.out.println("bound in new thread: " + USER.isBound()));
            thread.start();
            try {
                thread.join();
            } catch (InterruptedException e) {
                throw new RuntimeException(e);
            }
        });
    }
}
