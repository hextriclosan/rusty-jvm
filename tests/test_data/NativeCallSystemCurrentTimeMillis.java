package samples.nativecall.system;

public class NativeCallSystemCurrentTimeMillis {
    public static void main(String[] args) {
        long currentTimeMillis = System.currentTimeMillis();
        long nanoTime = System.nanoTime();
        System.out.println("{\"currentTimeMillis\": " + currentTimeMillis + ", \"nanoTime\": " + nanoTime + "}");
    }
}
