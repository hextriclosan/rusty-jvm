package samples.javacore.switches.trivial;

public class SwitchExample {

    public static void main(String[] args) {
        int value = 100_030;

        int lookupswitch = switch (value) {
            case 100_010 -> 100;
            case 100_020 -> 200;
            case 100_030 -> 300;
            default -> -1;
        };

        int tableswitch = switch (value) {
            case 100_029, 100_030, 100_031 -> 1000;
            case 100_032, 100_033, 100_034 -> 2000;
            default -> -100;
        };

        int result = lookupswitch + tableswitch;
    }
}
