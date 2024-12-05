package samples.arithmetics.sub.doubles;

public class SubDoubles {
    public static void main(String[] args) {
        double first = 1.23456789e200;
        double second = 1e201;
        double result = first - second;
        System.out.println(result);
    }
}
