package samples.npe.allnpeexamples;

import java.awt.Point;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.util.Objects;

public class AllNPEExamples {
    static void fieldAccess() {
        int placeholder0 = 10;
        int placeholder1 = 11;
        int placeholder2 = 12;
        int placeholder3 = 13;
        Point point = null;
        int x = point.x;
    }

    static void methodCallVirtual() {
        Object placeholder0 = null;
        String str = null;
        int len = str.length();
    }

    static void methodCallInterface() {
        Object placeholder0 = null;
        Runnable r = null;
        r.run();
    }

    static void methodCallOnParam(String first, String second) {
        int len = second.length();
    }

    static void arrayLength() {
        Object placeholder0 = null;
        int[] array = null;
        int x = array.length;
    }

    static void arrayAccess() {
        Object placeholder0 = null;
        String[] array = null;
        String x = array[0];
    }

    static void syncOnNull() {
        Object placeholder0 = null;
        Object lock = null;
        synchronized (lock) {
        }
    }

    static void unboxing() {
        Object placeholder0 = null;
        Integer integer = null;
        int primitive = integer;
    }

    static void throwNull() {
        Object placeholder0 = null;
        ArithmeticException e = null;
        throw e;
    }

    static void varargs() {
        Object placeholder0 = null;
        String[] strings = null;
        varargsMethod(strings);
    }
    static void varargsMethod(String... args) {
        System.out.println(args.length);
    }

    static void switchOnNull() {
        Object placeholder0 = null;
        String key = null;
        switch (key) {
            case "a" -> System.out.println("a");
        }
    }

    static void requireNonNull() {
        Object placeholder0 = null;
        Object object = null;
        Objects.requireNonNull(object);
    }

    static void requireNonNullWithMsg() {
        Object placeholder0 = null;
        Object object = null;
        Objects.requireNonNull(object, "Object must not be null");
    }

    static void methodRef() {
        Object placeholder0 = null;
        String str = null;
        Runnable r = str::toString;
        r.run();
    }

    static void reflectionField() throws Exception {
        Object placeholder0 = null;
        Field field = Point.class.getDeclaredField("x");
        Object object = null;
        Object o = field.get(object);// instance field, needs non-null
    }

    static void reflectionMethod() throws Exception {
        Object placeholder0 = null;
        Method method = String.class.getMethod("length");
        Object object = null;
        int len = (int) method.invoke(object); // instance method needs receiver
    }

    public static void main(String[] args) {
        Case[] tests = {
                new Case("Field access", AllNPEExamples::fieldAccess),
                new Case("Method call (invokevirtual)", AllNPEExamples::methodCallVirtual),
                new Case("Method call (invokeinterface)", AllNPEExamples::methodCallInterface),
                new Case("Method call on param", () -> methodCallOnParam(null, null)),
                new Case("Array length", AllNPEExamples::arrayLength),
                new Case("Array access", AllNPEExamples::arrayAccess),
                new Case("Synchronization on null", AllNPEExamples::syncOnNull),
                new Case("Unboxing", AllNPEExamples::unboxing),
                new Case("Throw null", AllNPEExamples::throwNull),
                new Case("Var args", AllNPEExamples::varargs),
                new Case("Switch on null", AllNPEExamples::switchOnNull),
                new Case("Objects.requireNonNull", AllNPEExamples::requireNonNull),
                new Case("Objects.requireNonNull with message", AllNPEExamples::requireNonNullWithMsg),
                new Case("Method reference bound to null", AllNPEExamples::methodRef),
                new Case("Reflection field get", () -> {
                    try {
                        reflectionField();
                    } catch (Exception e) {
                        throw new RuntimeException(e);
                    }
                }),
                new Case("Reflection method invoke", () -> {
                    try {
                        reflectionMethod();
                    } catch (Exception e) {
                        throw new RuntimeException(e);
                    }
                }),
        };

        for (Case test : tests) {
            try {
                test.npeExample().run();
            } catch (Throwable throwable) {
                System.out.println(test.description() + ": " + throwable.getMessage());
            }
        }
    }
}

record Case(String description, Runnable npeExample) {
}
