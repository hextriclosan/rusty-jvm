// javac -XDstringConcat=inline  -d . ExceptionExample.java
package samples.javacore.exceptionexample;

import java.util.Arrays;
import java.util.List;

public class ExceptionExample {
    public static void main(String[] args) {
        List<Case> cases = List.of(
                new FewTriesInOneMethod(),
                new TryWithResources(false),
                new TryWithResources(true),
                new TryWithResourcesMimic(false),
                new TryWithResourcesMimic(true),
                new ReThrowWithCause(),
                new FinallyIllustration()
        );
        for (Case aCase : cases) {
            aCase.run();
        }

    }
}

abstract class Case {
    private final String name = this.getClass().getSimpleName();

    public void run() {
        print("Running case: " + name);
        runImpl();
        print("");
    }

    protected abstract void runImpl();

    protected void print(String message) {
        System.out.println(message);
    }

    protected void print(String message, Throwable throwable) {
        String formattedMessage = String.format("%s: %s. cause=%s stackTrace=%s suppressed=%s", message, throwable, throwable.getCause(), Arrays.toString(throwable.getStackTrace()), Arrays.toString(throwable.getSuppressed()));
        print(formattedMessage);
    }

}

class FewTriesInOneMethod extends Case {
    @Override
    protected void runImpl() {
        try {
            print("Inside try block");
            thrower();
        } catch (NullPointerException e) {
            print("Caught as NullPointerException", e);
        } catch (RuntimeException e) {
            print("Caught as RuntimeException", e);
        } catch (Throwable e) {
            print("Caught as Throwable", e);
        }

        try {
            print("Inside another try block");
            throw new IndexOutOfBoundsException("This is an index out of bounds exception");
        } catch (Throwable e) {
            print("Caught as Throwable second time", e);
        }
    }

    private static void thrower() {
        throw new Error("This is an error");
    }
}

class TryWithResources extends Case {
    final boolean throwOnClose;

    public TryWithResources(boolean throwOnClose) {
        this.throwOnClose = throwOnClose;
    }

    @Override
    protected void runImpl() {
        try {
            print("Inside try-with-resources block");
            tryWithResourcesWithNoCatch();
        } catch (Exception e) {
            print("Caught try-with-resources exception", e);
        }
    }

    private void tryWithResourcesWithNoCatch() {
        try (CustomResource resource = new CustomResource(throwOnClose)) {
            resource.doSomethingAndThrow();
        }
    }
}

class TryWithResourcesMimic extends Case {
    final boolean throwOnClose;

    public TryWithResourcesMimic(boolean throwOnClose) {
        this.throwOnClose = throwOnClose;
    }

    @Override
    protected void runImpl() {
        try {
            print("Inside try-with-resources block");
            tryWithResourcesMimic();
        } catch (Exception e) {
            print("Caught try-with-resources exception", e);
        }
    }

    private void tryWithResourcesMimic() {
        CustomResource resource = new CustomResource(throwOnClose);

        try {
            resource.doSomethingAndThrow();
        } catch (Throwable firstThrowable) {
            try {
                resource.close();
            } catch (Throwable secondThrowable) {
                firstThrowable.addSuppressed(secondThrowable);
            }

            throw firstThrowable;
        }

        resource.close();
    }
}

class ReThrowWithCause extends Case {
    @Override
    protected void runImpl() {
        try {
            try {
                print("Inside try block");
                throw new RuntimeException("This is a runtime exception");
            } catch (Throwable e) {
                print("Caught as Throwable", e);
                throw new IllegalStateException(e);
            }
        } catch (IllegalStateException e) {
            print("Caught as IllegalStateException", e);
        }
    }
}

class FinallyIllustration extends Case {
    private String data;

    @Override
    protected void runImpl() {
        withoutException();
        print("No exception in try, finally still executes: " + data);
        print("============================");

        withCaughtException();
        print("Caught exception in try, finally still executes: " + data);
        print("============================");

        try {
            withUncaughtException();
            print("never reaches here");
        } catch (Throwable e) {
            print("Caught as Throwable", e);
            print("Uncaught exception in try, finally still executes: " + data);
        }
    }

    private void withoutException() {
        data = "";
        try {
            print("Executing try block");
            data += "try-";
        } finally {
            print("Executing finally block");
            data += "finally";
        }
    }

    private void withCaughtException() {
        data = "";
        try {
            print("Executing try block");
            data += "try-";
            throw new RuntimeException("Exception in try");
        } catch (IllegalArgumentException e) {
            data += "catch0-";
        } catch (Throwable e) {
            print("Executing catch block" + e);
            data += "catch-";
        } finally {
            print("Executing finally block");
            data += "finally";
        }
    }

    private void withUncaughtException() {
        data = "";
        try {
            print("Executing try block");
            data += "try-";
            throw new RuntimeException("Exception in try");
        } catch (IllegalArgumentException e) {
            print("Executing catch block", e);
            data += "catch-";
        } finally {
            print("Executing finally block");
            data += "finally";
        }
    }
}

class CustomResource implements AutoCloseable {
    final boolean throwOnClose;

    public CustomResource(boolean throwOnClose) {
        this.throwOnClose = throwOnClose;
    }

    public void doSomethingAndThrow() {
        System.out.println("Doing something with the resource");
        throw new RuntimeException("An error occurred while using the resource");
    }

    @Override
    public void close() {
        System.out.println("Custom resource is about to be closed");
        if (throwOnClose) {
            throw new IllegalStateException("An error occurred while closing the resource");
        }
        System.out.println("Custom resource is closed");
    }
}
