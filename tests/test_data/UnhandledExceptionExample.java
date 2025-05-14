package samples.javacore.unhandledexception;

public class UnhandledExceptionExample {
    public static void main(String[] args) {
        fun1();
    }

    private static void fun1() {
        fun2();
    }

    private static void fun2() {
        fun3();
    }

    private static void fun3() {
        String s = "hello";
        System.out.println(s.substring(2, 1)); // end < begin
        System.out.println("This line will not be reached");
    }
}
