
public class DependsOnDependable {
    static int valueB = Dependable.valueA + 50;

    static {
        valueB += 100;
    }
}
