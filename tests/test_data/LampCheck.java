public class LampCheck {
    static class Light {
    }

    static class Lamp extends Light {
    }

    static class LightOutException extends RuntimeException {
    }

    public static void main(String[] args) {
        Object lamp = new Lamp();
        Light light = (Light) lamp;
        System.out.println("cast to superclass: " + (light == lamp));
        System.out.println("instanceof: " + (lamp instanceof Light));

        try {
            throw new LightOutException();
        } catch (LightOutException exception) {
            System.out.println("caught: " + exception.getClass().getName());
        }

        Object plainLight = new Light();
        try {
            Lamp wrong = (Lamp) plainLight;
        } catch (ClassCastException exception) {
            System.out.println(exception.getMessage());
        }
    }
}
