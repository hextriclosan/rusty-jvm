package samples.javacore.assertions.status;

import samples.javacore.assertions.status.sub.SubPackageAssertions;

public class AssertionStatusExample {
    public static void main(String[] args) {
        report("AssertionStatusExample", AssertionStatusExample.class, assertionsRun());
        report("SamePackageAssertions", SamePackageAssertions.class, SamePackageAssertions.assertionsRun());
        report("sub.SubPackageAssertions", SubPackageAssertions.class, SubPackageAssertions.assertionsRun());
        report("AssertionStatusExample.Nested", Nested.class, Nested.assertionsRun());
        report("String", String.class);
        report("int", int.class);
        report("int[]", int[].class);

        // From here on the class loader decides, starting from the options it gets from the VM
        ClassLoader loader = AssertionStatusExample.class.getClassLoader();
        loader.setClassAssertionStatus("samples.javacore.assertions.status.LateAssertions", true);
        System.out.println("-- after setClassAssertionStatus(LateAssertions, true)");
        report("AssertionStatusExample", AssertionStatusExample.class);
        report("SamePackageAssertions", SamePackageAssertions.class);
        report("sub.SubPackageAssertions", SubPackageAssertions.class);
        report("AssertionStatusExample.Nested", Nested.class);
        report("LateAssertions", LateAssertions.class, LateAssertions.assertionsRun());
    }

    static boolean assertionsRun() {
        boolean run = false;
        assert run = true;
        return run;
    }

    static class Nested {
        static boolean assertionsRun() {
            boolean run = false;
            assert run = true;
            return run;
        }
    }

    private static void report(String name, Class<?> clazz) {
        System.out.println(name + ": desired " + clazz.desiredAssertionStatus());
    }

    private static void report(String name, Class<?> clazz, boolean run) {
        System.out.println(name + ": desired " + clazz.desiredAssertionStatus() + ", assert run " + run);
    }
}

class SamePackageAssertions {
    static boolean assertionsRun() {
        boolean run = false;
        assert run = true;
        return run;
    }
}

    class LateAssertions {
    static boolean assertionsRun() {
        boolean run = false;
        assert run = true;
        return run;
    }
}
