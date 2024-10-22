package samples.javacore.switches.trivial;

public class SwitchExample {

    public static void main(String[] args) {
        int value = 30;
        int result = switched(value);
    }

    private static int switched(int value) {
        int result;
        switch (value) {
            case 10:
                result = 100;
                break;
            case 20:
                result = 200;
                break;
            case 30:
                result = 300;
                break;
            default:
                result = -1;
                break;
        }

        return result;
    }

}
