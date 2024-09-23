package samples.fields.staticinitialization.array;

public class StaticInitializationArray {
    private static final int STATIC_FINAL_FIELD = init();
    private static int staticField1;
    private static int staticField2;
    private static int[] staticArray;

    // Static initializer block
    static {
        staticField1 = 5; // Direct static field initialization
        staticArray = new int[staticField1]; // Initialize array in static block
        for (int i = 0; i < staticArray.length; i++) {
            staticArray[i] = i * staticField1;
        }
    }

    private static int init() {
        return 42;
    }

    public static void main(String[] args) {
        int arraySum = 0;
        for (int i = 0; i < staticArray.length; i++) {
            arraySum += staticArray[i];
        }

        int result = STATIC_FINAL_FIELD + staticField1 + staticField2 + arraySum;
    }

    // Another static block (executed in order of appearance)
    static {
        staticField1 += 50; // Modify staticField1
        staticField2 = staticField1 * 2; // Initialize staticField2 based on staticField1
    }
}
