package samples.fields.staticinitialization.advanced;

public class StaticInitializationAdvanced {
    public static void main(String[] args) {
        int classCWithHelperNonStaticGetter = ClassC.staticField;

        int classDWithHelperStaticGetter = ClassD.staticField;

        ClassC.staticField = 600;
        int classCIsModified = ClassC.staticField;

        int classEAsSumOfCAndD = ClassE.staticField;

        int result = classCWithHelperNonStaticGetter + classDWithHelperStaticGetter * classCIsModified / classEAsSumOfCAndD;
        System.out.println(result);
    }
}

class ClassC {
    static Helper helper;
    static int staticField;

    static {
        helper = new Helper(500);
        staticField = 150 + helper.calculateNonStaticFieldValue();
    }
}

class ClassD {
    static int staticField;

    static {
        staticField = Helper.calculateStaticFieldValue();
    }
}

class ClassE {
    static int staticField;

    static {
        staticField = ClassC.staticField + ClassD.staticField;
    }
}

class Helper {
    private final int value;

    Helper(int value) {
        this.value = value;
    }

    static int calculateStaticFieldValue() {
        return 250;
    }

    public int calculateNonStaticFieldValue() {
        return value;
    }
}
