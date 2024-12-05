package samples.arithmetics.adder.longs;

public class AdderLong {

    public static void main(String[] args) {
        long result = add(
                42_949_672_980L/*h=10,l=20*/,
                128_849_018_920L/*h=30,l=40*/
        );
        System.out.println(result);
    }

    private static long add(long a, long b) {
        return a + b;
    }

}
