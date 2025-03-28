package samples.javacore.recordexample;

import java.util.Objects;

public class RecordExample {
    public static void main(String[] args) {
        Rcd one = new Rcd(10, 20);
        Rcd two = new Rcd(100, 200);
        Rcd three = new Rcd(10, 20);

        System.out.println(one); // Unknown reference opcode: 186
        System.out.println(two.second());
        System.out.println(Objects.equals(one, three)); // Unknown reference opcode: 186
    }
}

record Rcd(int first, long second) {
}
