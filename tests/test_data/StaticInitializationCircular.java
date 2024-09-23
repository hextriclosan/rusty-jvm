package samples.fields.staticinitialization.circular;

public class StaticInitializationCircular {
    public static void main(String[] args) {
        int classA = ClassACircular.staticField;
        int classB = ClassBCircular.staticField;
        int result = classA + classB;
    }
}

class ClassACircular {
    static int staticField;

    static {
        staticField = 100 + ClassBCircular.staticField;
    }
}

class ClassBCircular {
    static int staticField;

    static {
        staticField = 300 + ClassACircular.staticField;
    }
}
