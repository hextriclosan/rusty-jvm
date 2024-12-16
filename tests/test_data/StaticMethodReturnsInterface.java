package samples.inheritance.staticmethodreturnsinterface;

import java.util.Set;

public class StaticMethodReturnsInterface {

    public static void main(String[] args) {
        Set<String> set = getSet();
        System.out.println(set);
    }

    private static Set<String> getSet() {
        return Set.of("some string");
    }
}
