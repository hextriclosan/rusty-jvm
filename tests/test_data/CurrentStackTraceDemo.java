package samples.concurrency.threads;

public class CurrentStackTraceDemo {
    public static void main(String[] args) {
        level1();
    }
    static void level1() { level2(); }
    static void level2() {
        for (StackTraceElement e : Thread.currentThread().getStackTrace()) {
            System.out.println(e.getClassName() + "." + e.getMethodName());
        }
    }
}
