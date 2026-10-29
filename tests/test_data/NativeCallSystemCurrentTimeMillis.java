package samples.nativecall.system;

public class NativeCallSystemCurrentTimeMillis {
    private static final long SLEEP_MILLIS = 50;

    public static void main(String[] args) throws InterruptedException {
        long currentTimeMillis = System.currentTimeMillis();
        long nanoTime = System.nanoTime();

        // nanoTime has an arbitrary origin, so only differences between readings are meaningful
        boolean nanoTimeMonotonic = true;
        long previous = System.nanoTime();
        for (int i = 0; i < 1000; i++) {
            long current = System.nanoTime();
            if (current < previous) {
                nanoTimeMonotonic = false;
            }
            previous = current;
        }

        long before = System.nanoTime();
        Thread.sleep(SLEEP_MILLIS);
        long elapsedNanos = System.nanoTime() - before;
        boolean nanoTimeMeasuresSleep = elapsedNanos >= SLEEP_MILLIS * 1_000_000;

        System.out.println("{\"currentTimeMillis\": " + currentTimeMillis
                + ", \"nanoTime\": " + nanoTime
                + ", \"nanoTimeMonotonic\": " + nanoTimeMonotonic
                + ", \"nanoTimeMeasuresSleep\": " + nanoTimeMeasuresSleep + "}");
    }
}
