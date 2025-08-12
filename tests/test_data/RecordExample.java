package samples.javacore.recordexample;

public class RecordExample {
    public static void main(String[] args) {
        Rcd one = new Rcd(10, 20L);
        Rcd two = new Rcd(100, 200L);
        Rcd three = new Rcd(10, 20L);
        System.out.println("one: " + one);
        System.out.println("two: " + two);
        System.out.println("three: " + three);
        int hashOne = one.hashCode();
        int hashTwo = two.hashCode();
        int hashThree = three.hashCode();
        System.out.println("hashOne: " + hashOne);
        System.out.println("hashTwo: " + hashTwo);
        System.out.println("hashThree: " + hashThree);

        boolean oneEqTwo = one.equals(two);
        System.out.printf("%s equals %s: %b%n", one, two, oneEqTwo);

        boolean oneEqThree = one.equals(three);
        System.out.printf("%s equals %s: %b%n", one, three, oneEqThree);
    }
}

record Rcd(Integer first, Long second) {
}
// record Rcd(Integer first, long second) { // bug: `second` is 85899345920 instead of 20
// }
