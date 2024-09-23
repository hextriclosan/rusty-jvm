package samples.fields.staticinitialization.oneclass;

public class StaticInitializationWithinOneClass {
    static int staticField;

    static {
        int xxx = 1337;
        staticField = xxx;
    }

    public static void main(String[] args) {
        staticField = 100;
        int result = staticField;
    }
}
