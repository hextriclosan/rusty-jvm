
public class AdderNegativeLong {

    public static void main(String[] args) {
         long result = add(-1_000_000_000_000_000L, -990_000_000_000_000L);
    }

    private static long add(long a, long b) {
        return a + b;
    }

}