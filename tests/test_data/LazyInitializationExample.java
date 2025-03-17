package samples.fields.staticinitialization.lazy;

class LazyInitializationExample {
    static {
        System.out.println("LazyInitializationExample class loaded.");
    }

    static void triggerSomethingElse() {
        System.out.println("Triggering another method, but NOT accessing LazyHolder.");
    }

    static class LazyHolder {
        static {
            System.out.println("LazyHolder class loaded.");
        }

        static final String VALUE = initialize();

        private static String initialize() {
            System.out.println("Initializing VALUE...");
            return "Lazy Loaded Value";
        }
    }

    public static void main(String[] args) {
        System.out.println("Before accessing LazyHolder.VALUE");

        triggerSomethingElse();
        // LazyHolder should NOT be loaded here

        System.out.print("Now accessing LazyHolder.VALUE: ");
        System.out.println(LazyHolder.VALUE);
        // LazyHolder should be loaded now, triggering its static block

        System.out.print("Accessing LazyHolder.VALUE again: ");
        System.out.println(LazyHolder.VALUE);
        // VALUE should already be initialized, so no reinitialization
    }
}
