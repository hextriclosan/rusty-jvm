package samples.perf.userperfcounterexample;

import jdk.internal.perf.Perf;
import jdk.internal.perf.PerfCounter;

import java.io.IOException;
import java.nio.ByteBuffer;

import sun.management.counter.Units;
import sun.management.counter.Variability;

import static jdk.internal.perf.PerfCounter.newPerfCounter;

public class UserPerfCounterExample {
    public static void main(String[] args) throws IOException {
        // Long perf counter
        PerfCounter counter = newPerfCounter("my.counter");
        System.out.println(counter);

        counter.set(10);
        System.out.println(counter);

        counter.add(100);
        System.out.println(counter);

        counter.increment();
        System.out.println(counter);

        // String perf counter
        Perf perf = Perf.getPerf();
        String value = "hello-perf";
        ByteBuffer byteBuffer = perf.createString("my.string", Variability.VARIABLE.intValue(), Units.STRING.intValue(), value);

        System.out.println("Created string: " + readCString(byteBuffer)); // todo: validate this with jcmd utility
    }

    private static String readCString(ByteBuffer bb) {
        int capacity = bb.capacity();

        byte[] tmp = new byte[capacity];

        for (int i = 0; i < capacity; i++) {
            tmp[i] = bb.get(i);
        }

        int len = 0;
        while (len < tmp.length && tmp[len] != 0) {
            len++;
        }

        return new String(tmp, 0, len);
    }
}
