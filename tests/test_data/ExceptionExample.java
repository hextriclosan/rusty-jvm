// javac -XDstringConcat=inline  -d . ExceptionExample.java
package samples.javacore.exceptionexample;

import java.util.List;

public class ExceptionExample {
    public static void main(String[] args) {
        System.out.println("Beginning of main");
        List<Case> cases = List.of(new FewTriesInOneMethod());
        for (Case aCase: cases) {
            aCase.run();
        }
        System.out.println("End of main");
    }
}

abstract class Case {
    private final String name = this.getClass().getSimpleName();
    public void run() {
        System.out.println("Running case: " + name);
        runImpl();
        System.out.println();
    }

    protected abstract void runImpl();

    protected void print(String message) {
        System.out.println("  " + message);
    }

}

class FewTriesInOneMethod extends Case {
    @Override
    protected void runImpl() {
        try {
            print("Inside try block");
            thrower();
            print("Should be never reached");
        } catch (NullPointerException e) {
            print("Caught as NullPointerException: " + e);
        } catch (RuntimeException e) {
            print("Caught as RuntimeException: " + e);
        } catch (Throwable e) {
            print("Caught as Throwable: " + e);
        }

        try {
            print("Inside another try block");
            throw new IndexOutOfBoundsException("This is an index out of bounds exception");
        } catch (Throwable e) {
            print("Caught as Throwable second time: " + e);
        }
    }

    private static void thrower() {
        throw new Error("This is an error");
    }
}
