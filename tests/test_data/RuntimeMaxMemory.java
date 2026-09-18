package samples.runtime.maxmemory;

public class RuntimeMaxMemory {
    public static void main(String[] args) {
        long maxMemory = Runtime.getRuntime().maxMemory();
        System.out.println("positive=" + (maxMemory > 0));
        System.out.println("finite=" + (maxMemory < Long.MAX_VALUE));
    }
}
