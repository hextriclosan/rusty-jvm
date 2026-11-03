package samples.runtime.maxmemory;

public class RuntimeMaxMemory {
    public static void main(String[] args) {
        Runtime runtime = Runtime.getRuntime();
        long maxMemory = runtime.maxMemory();
        long totalMemory = runtime.totalMemory();
        long freeMemory = runtime.freeMemory();
        System.out.println("positive=" + (maxMemory > 0));
        System.out.println("finite=" + (maxMemory < Long.MAX_VALUE));
        System.out.println("total within max=" + (totalMemory <= maxMemory));
        System.out.println("free within total=" + (freeMemory <= totalMemory));
    }
}
