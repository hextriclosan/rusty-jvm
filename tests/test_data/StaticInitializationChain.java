package samples.fields.staticinitialization.chain;

public class StaticInitializationChain {
    public static void main(String[] args) {
        int result = DependsOnDependable.valueB;
        System.out.println(result);
    }
}

class DependsOnDependable {
    static int valueB = Dependable.valueA + 50;

    static {
        valueB += 100;
    }
}

class Dependable {
    static int valueA = 100;

    static {
        valueA = 200;
    }
}
