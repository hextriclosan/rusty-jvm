package samples.javacore.switches.trivial;

public class SwitchExample {

    public static void main(String[] args) {
        int value = 30;

        int lookupswitch = switch (value) {
            case 10 -> 100;
            case 20 -> 200;
            case 30 -> 300;
            default -> -1;
        };

        int tableswitch = switch (value) {
            case 29, 30, 31 -> 1000;
            case 32, 33, 34 -> 2000;
            default -> -100;
        };

        int result = lookupswitch + tableswitch;
    }
}
