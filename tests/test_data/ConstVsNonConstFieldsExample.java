package samples.staticinit.const_vs_nonconst_fields;

public class ConstVsNonConstFieldsExample {
    public static void main(String[] args) {
        System.out.println("CONST_INT_FIELD: " + Const1.CONST_INT_FIELD);
        System.out.println("CONST_STRING_FIELD: " + Const2.CONST_STRING_FIELD);
        System.out.println("COMPILE_TIME_CONST_FILED: " + Const3.COMPILE_TIME_CONST_FILED);
        System.out.println("CONST_FILED_FROM_ANOTHER_CLASS: " + Const4.CONST_FILED_FROM_ANOTHER_CLASS);

        System.out.println("NON_CONST_INT_FIELD: " + NonConst1.NON_CONST_INT_FIELD);
        System.out.println("NON_CONST_STRING_FIELD: " + NonConst2.NON_CONST_STRING_FIELD);
        System.out.println("RUNTIME_INITIALIZED_NON_CONST_FILED: " + NonConst3.RUNTIME_INITIALIZED_NON_CONST_FILED);
        System.out.println("NON_CONST_FILED_DEPENDS_ON_ANOTHER: " + NonConst4.NON_CONST_FILED_DEPENDS_ON_ANOTHER);
        System.out.println("NON_CONST_FILED_DEPENDS_ON_ANOTHER_CLASS: " + NonConst5.NON_CONST_FILED_DEPENDS_ON_ANOTHER_CLASS);
        System.out.println("NON_CONST_FILED_CONDITIONAL: " + NonConst6.NON_CONST_FILED_CONDITIONAL);
        System.out.println("NON_CONST_FILED_ASSIGNED_IN_STATIC_BLOCK: " + NonConst7.NON_CONST_FILED_ASSIGNED_IN_STATIC_BLOCK);
    }
}

class Const1 {
    static {
        System.out.println("Const1 static block");
    }

    static final int CONST_INT_FIELD = 1337;
}

class Const2 {
    static {
        System.out.println("Const2 static block");
    }

    static final String CONST_STRING_FIELD = "Hello, Constant!";
}

class Const3 {
    static {
        System.out.println("Const3 static block");
    }

    private static final int RADIUS = 2;
    static final double COMPILE_TIME_CONST_FILED = 3.14 * RADIUS * RADIUS;
}

class Const4 {
    static {
        System.out.println("Const4 static block");
    }

    static final double CONST_FILED_FROM_ANOTHER_CLASS = Const3.COMPILE_TIME_CONST_FILED * 2;
}


class NonConst1 {
    static {
        System.out.println("NonConst1 static block");
    }

    static int NON_CONST_INT_FIELD = 1337;
}

class NonConst2 {
    static {
        System.out.println("NonConst2 static block");
    }

    static String NON_CONST_STRING_FIELD = "Hello, Non-Constant!";
}

class NonConst3 {
    static {
        System.out.println("NonConst3 static block");
    }

    static final Integer RUNTIME_INITIALIZED_NON_CONST_FILED = 42;
}

class NonConst4 {
    static {
        System.out.println("NonConst4 static block");
    }

    private static int RADIUS = 2;
    static final double NON_CONST_FILED_DEPENDS_ON_ANOTHER = 3.14 * RADIUS * RADIUS;
}

class NonConst5 {
    static {
        System.out.println("NonConst5 static block");
    }

    static final double NON_CONST_FILED_DEPENDS_ON_ANOTHER_CLASS = NonConst4.NON_CONST_FILED_DEPENDS_ON_ANOTHER * 2;
}

class NonConst6 {
    static {
        System.out.println("NonConst6 static block");
    }

    static final int NON_CONST_FILED_CONDITIONAL = "hello".length() == 5 ? 10 : 20;
}

class NonConst7 {
    static {
        System.out.println("NonConst7 static block");
        NON_CONST_FILED_ASSIGNED_IN_STATIC_BLOCK = 1337;
    }

    static final int NON_CONST_FILED_ASSIGNED_IN_STATIC_BLOCK;
}
