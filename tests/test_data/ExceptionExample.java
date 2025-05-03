// javac -XDstringConcat=inline  -d . ExceptionExample.java
package samples.javacore.exceptionexample;

public class ExceptionExample {
    public static void main(String[] args) {
        System.out.println("Beginning of main");
        try {
            System.out.println("Inside try block");
            thrower();
            System.out.println("Should be never reached");
        } catch (NullPointerException e) {
            System.out.println("Caught as NullPointerException: " + e);
        } catch (RuntimeException e) {
            System.out.println("Caught as RuntimeException: " + e);
        } catch (Throwable e) {
            System.out.println("Caught as Throwable: " + e);
        }
        System.out.println("End of main");
    }

    private static void thrower() {
        throw new Error();
    }
}
