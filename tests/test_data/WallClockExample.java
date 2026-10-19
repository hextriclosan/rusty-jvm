package samples.nativecall.system.wallclock;

import java.time.Clock;
import java.time.Instant;

public class WallClockExample {
    // The wall clock may be stepped (e.g. by NTP) between independent reads, so compare with some
    // slack; reading it wrong (wrong unit or offset) is off by far more than that
    private static final long SLACK_MILLIS = 1000;

    public static void main(String[] args) {
        long before = System.currentTimeMillis();
        Instant instant = Instant.now();
        Instant clockInstant = Clock.systemUTC().instant();
        long after = System.currentTimeMillis();

        System.out.println("Instant.now() agrees with currentTimeMillis: " + within(instant, before, after));
        System.out.println("Clock.systemUTC() agrees with currentTimeMillis: " + within(clockInstant, before, after));
    }

    private static boolean within(Instant instant, long fromMillis, long toMillis) {
        long millis = instant.toEpochMilli();
        return millis >= fromMillis - SLACK_MILLIS && millis <= toMillis + SLACK_MILLIS;
    }
}
