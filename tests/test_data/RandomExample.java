package samples.javautil.random;

import java.util.Random;

public class RandomExample {
    public static void main(String[] args) {
        long seed = -466210000;
        Random random = new Random(seed); // Native Call Error: Native method java/lang/Class:getDeclaredFields0:(Z)[Ljava/lang/reflect/Field; not found
        System.out.println(random.nextInt());
    }
}
