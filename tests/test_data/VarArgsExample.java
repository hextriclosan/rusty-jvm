package samples.javacore.varargs.trivial;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class VarArgsExample {
    public static void main(String[] args) {
        VarArgsExample varArgsExample = new VarArgsExample();
        List<Integer> list = new ArrayList<>();
        list.add(1337);
        Map<Integer, Integer> map = new HashMap<>();
        map.put(42, 1);
        varArgsExample.varArgs(4, 1000_000_000_000L, 3.14, list, map);
    }

    private void varArgs(Object... args) {
        for (Object arg : args) {
            System.out.println(arg);
        }
    }
}
