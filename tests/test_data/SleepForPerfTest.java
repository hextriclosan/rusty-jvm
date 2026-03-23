package samples.perf;

public class SleepForPerfTest {
    public static void main(String[] args) throws Exception {
        // Block until stdin is closed by the test harness.
        // This keeps the process alive so the test can inspect the perf file.
        System.in.read();
        System.out.println("perf sleep test done");
    }
}
