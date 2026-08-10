package samples.reflection.methodhandlewidefields;

import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;

/**
 * Category-2 fields (long, double) read and written through MethodHandles.
 *
 * <p>Every case pairs a MethodHandle access with a direct field access. That pairing is the point:
 * a MethodHandle write followed by a MethodHandle read round-trips correctly even when both swap
 * the two halves of a wide value, so a test written purely in MethodHandles cannot see the fault.
 * The int cases are controls — a narrow value is a single slot, so chunk order cannot affect it.
 */
public class MethodHandleWideFields {
    long instanceLong;
    double instanceDouble;
    int instanceInt;
    static long staticLong;
    static double staticDouble;

    /** Halves differ, so a swap is visible rather than symmetric. */
    private static final long PATTERN = 0x0123456789ABCDEFL;

    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        MethodHandleWideFields o = new MethodHandleWideFields();

        o.instanceLong = PATTERN;
        MethodHandle getLong =
                lookup.findGetter(MethodHandleWideFields.class, "instanceLong", long.class);
        System.out.println("instance long get:   " + Long.toHexString((long) getLong.invokeExact(o)));

        MethodHandle setLong =
                lookup.findSetter(MethodHandleWideFields.class, "instanceLong", long.class);
        setLong.invokeExact(o, -PATTERN);
        System.out.println("instance long set:   " + Long.toHexString(o.instanceLong));

        o.instanceDouble = 12.5d;
        MethodHandle getDouble =
                lookup.findGetter(MethodHandleWideFields.class, "instanceDouble", double.class);
        System.out.println("instance double get: " + (double) getDouble.invokeExact(o));

        MethodHandle setDouble =
                lookup.findSetter(MethodHandleWideFields.class, "instanceDouble", double.class);
        setDouble.invokeExact(o, -0.75d);
        System.out.println("instance double set: " + o.instanceDouble);

        staticLong = PATTERN;
        MethodHandle getStaticLong =
                lookup.findStaticGetter(MethodHandleWideFields.class, "staticLong", long.class);
        System.out.println("static long get:     " + Long.toHexString((long) getStaticLong.invokeExact()));

        MethodHandle setStaticLong =
                lookup.findStaticSetter(MethodHandleWideFields.class, "staticLong", long.class);
        setStaticLong.invokeExact(-PATTERN);
        System.out.println("static long set:     " + Long.toHexString(staticLong));

        staticDouble = 12.5d;
        MethodHandle getStaticDouble =
                lookup.findStaticGetter(MethodHandleWideFields.class, "staticDouble", double.class);
        System.out.println("static double get:   " + (double) getStaticDouble.invokeExact());

        MethodHandle setStaticDouble =
                lookup.findStaticSetter(MethodHandleWideFields.class, "staticDouble", double.class);
        setStaticDouble.invokeExact(-0.75d);
        System.out.println("static double set:   " + staticDouble);

        o.instanceInt = 42;
        MethodHandle getInt =
                lookup.findGetter(MethodHandleWideFields.class, "instanceInt", int.class);
        System.out.println("instance int get:    " + (int) getInt.invokeExact(o));

        MethodHandle setInt =
                lookup.findSetter(MethodHandleWideFields.class, "instanceInt", int.class);
        setInt.invokeExact(o, 7);
        System.out.println("instance int set:    " + o.instanceInt);
    }
}
