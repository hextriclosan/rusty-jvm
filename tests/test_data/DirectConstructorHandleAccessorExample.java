package samples.reflection.directconstructorhandleaccessor;

import java.lang.reflect.Constructor;
import java.lang.reflect.InvocationTargetException;
import java.util.List;
import java.util.Map;

public class DirectConstructorHandleAccessorExample {
    public static void main(String[] args) throws Exception {
        constructAndPrint();
        constructWithNullAndPrint();
    }

    private static void constructAndPrint() throws InvocationTargetException, InstantiationException, IllegalAccessException, NoSuchMethodException {
        // 1) Grab a private constructor
        Constructor<Target> targetConstructor = Target.class.getDeclaredConstructor(
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
                Map.class);

        // 2) Make it accessible (bypasses Java access checks)
        targetConstructor.setAccessible(true);

        // 3) Invoke it — under the hood this uses
        //    jdk/internal/reflect/DirectConstructorHandleAccessor$NativeAccessor.newInstance0(...)
        Target target = targetConstructor.newInstance(
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
                Map.of("8", "eight"));
        System.out.println(target);
    }

    private static void constructWithNullAndPrint() throws InvocationTargetException, InstantiationException, IllegalAccessException, NoSuchMethodException {
        Constructor<Target> targetConstructor = Target.class.getDeclaredConstructor();
        targetConstructor.setAccessible(true);
        Target target = targetConstructor.newInstance((Object[])null);
        System.out.println(target);
    }
}

final class Target {
    private final byte byteField;
    private final boolean booleanField;
    private final short shortField;
    private final char charField;
    private final int intField;
    private final float floatField;
    private final long longField;
    private final double doubleField;
    private final String stringField;
    private final Object objectField;
    private final Map<String, String> mapField;

    private Target(byte byteField, boolean booleanField, short shortField, char charField, int intField, float floatField, long longField, double doubleField, String stringField, Object objectField, Map<String, String> mapField) {
        this.byteField = byteField;
        this.booleanField = booleanField;
        this.shortField = shortField;
        this.charField = charField;
        this.intField = intField;
        this.floatField = floatField;
        this.longField = longField;
        this.doubleField = doubleField;
        this.stringField = stringField;
        this.objectField = objectField;
        this.mapField = mapField;
    }

    private Target() {
        this(Byte.MAX_VALUE,
                             true,
                             Short.MAX_VALUE,
                             'є',
                             Integer.MAX_VALUE,
                             1.337f,
                             Long.MAX_VALUE,
                             Math.PI,
                             "seventy",
                             List.of(10, 20, 30),
                             Map.of("80", "eighty"));
    }

    @Override
    public String toString() {
        final StringBuilder sb = new StringBuilder("Target{");
        sb.append("byteField=").append(byteField);
        sb.append(", booleanField=").append(booleanField);
        sb.append(", shortField=").append(shortField);
        sb.append(", charField=").append(charField);
        sb.append(", intField=").append(intField);
        sb.append(", floatField=").append(floatField);
        sb.append(", longField=").append(longField);
        sb.append(", doubleField=").append(doubleField);
        sb.append(", stringField='").append(stringField).append('\'');
        sb.append(", objectField=").append(objectField);
        sb.append(", mapField=").append(mapField);
        sb.append('}');
        return sb.toString();
    }
}
