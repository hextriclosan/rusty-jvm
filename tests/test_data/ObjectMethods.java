package samples.javacore.object.trivial;

public class ObjectMethods implements Interface {
    public static void main(String[] args) {
        ObjectMethods first = new ObjectMethods();
        ObjectMethods second = new ObjectMethods();
        Interface third = new ObjectMethods();

        Class<? extends ObjectMethods> firstClass = first.getClass();
        Class<? extends ObjectMethods> secondClass = second.getClass();
        Class<? extends Interface> thirdClass = third.getClass();

        int result = firstClass == secondClass
                && secondClass == ObjectMethods.class
                && thirdClass == ObjectMethods.class
                ? 1
                : 0;
    }
}

interface Interface {
}
