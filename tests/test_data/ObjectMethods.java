package samples.javacore.object.trivial;

public class ObjectMethods {
    public static void main(String[] args) {
        ObjectMethods first = new ObjectMethods();
        ObjectMethods second = new ObjectMethods();

        Class<? extends ObjectMethods> firstClass = first.getClass();
        Class<? extends ObjectMethods> secondClass = second.getClass();

        int result = firstClass == secondClass && secondClass == ObjectMethods.class ? 1 : 0;
    }
}
