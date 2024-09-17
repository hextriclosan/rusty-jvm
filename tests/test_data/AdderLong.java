//javac -g -parameters Adder.java

public class AdderLong {

    public static void main(String[] args) {
         long result = add(100_000_000_000L, -99_000_000_000L);
    }

    private static long add(long a, long b) {
        return a + b;
    }

}
