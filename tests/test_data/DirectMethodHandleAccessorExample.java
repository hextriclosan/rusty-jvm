package samples.reflection.directmethodhandleaccessor;

import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;
import java.util.List;
import java.util.Map;

public class DirectMethodHandleAccessorExample {
    public static void main(String[] args) throws Exception {
        invokeAndPrint();
    }

    private static void invokeAndPrint() throws NoSuchMethodException,
            InvocationTargetException, IllegalAccessException {
        // 1) Create a plain target object
        Target target = new Target("TargetInstance");

        // 2) Grab the private method with many parameters
        Method secretMethod = Target.class.getDeclaredMethod(
                "secretMethod",
                byte.class,
                boolean.class,
                short.class,
                char.class,
                int.class,
                float.class,
                long.class,
                double.class,
                String.class,
                Object.class,
                Map.class
        );

        // 3) Make it accessible
        secretMethod.setAccessible(true);

        // 4) Invoke it reflectively — this uses DirectMethodHandleAccessor
        String result = (String) secretMethod.invoke(
                target,
                Byte.MIN_VALUE,
                true,
                Short.MIN_VALUE,
                'ї',
                2_000_000_000,
                3.14f,
                Long.MIN_VALUE,
                Math.PI,
                "seven",
                List.of(1, 2, 3),
                Map.of("8", "eight")
        );

        System.out.println(result);
    }
}

final class Target {
    private final String name;

    public Target(String name) {
        this.name = name;
    }

    private String secretMethod(
            byte byteParam,
            boolean booleanParam,
            short shortParam,
            char charParam,
            int intParam,
            float floatParam,
            long longParam,
            double doubleParam,
            String stringParam,
            Object objectParam,
            Map<String, String> mapParam
    ) {
        return name + " - secretMethod invoked with: " +
                "byte=" + byteParam +
                ", boolean=" + booleanParam +
                ", short=" + shortParam +
                ", char=" + charParam +
                ", int=" + intParam +
                ", float=" + floatParam +
                ", long=" + longParam +
                ", double=" + doubleParam +
                ", string=" + stringParam +
                ", object=" + objectParam +
                ", map=" + mapParam;
    }
}
