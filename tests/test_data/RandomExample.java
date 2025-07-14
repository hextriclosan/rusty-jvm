package samples.javautil.random;

import java.util.Random;

public class RandomExample {
    public static void main(String[] args) {
        long seed = -466210000;
        Random random = new Random(seed);
        System.out.println(random.nextInt());
    }
}
