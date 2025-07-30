package samples.javacore.invokedynamic.lambda;

import java.util.function.BiFunction;

public class LambdaExample {
    public static void main(String[] args) {
        int i = operation(Integer::sum, 5, 10);
        //System.out.println(i);

        //Runnable runnable = () -> { int xxx = 42;};

    }

    private static int operation(BiFunction<Integer, Integer, Integer> func, int a, int b) {
        return func.apply(a, b);
    }
}
