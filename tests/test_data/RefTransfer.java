package samples.gc;

import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.invoke.VarHandle;
import java.lang.reflect.InvocationTargetException;

public class RefTransfer implements Cloneable {
    public Object value;

    public static Object mixed(Object object, int number, long wide, double decimal, Object[] array) {
        require(number == 7 && wide == 0x1122334455667788L && decimal == 3.25);
        require(array[0] == object);
        return object;
    }

    public static void raise(RuntimeException exception) {
        throw exception;
    }

    private static void require(boolean condition) {
        if (!condition) {
            throw new AssertionError("reference transfer");
        }
    }

    public static void main(String[] args) throws Throwable {
        Object object = new Object();
        Object replacement = new Object();
        Object[] array = {object};
        long wide = 0x1122334455667788L;
        require(mixed(object, 7, wide, 3.25, array) == object);

        MethodHandles.Lookup lookup = MethodHandles.lookup();
        MethodType type = MethodType.methodType(
            Object.class, Object.class, int.class, long.class, double.class, Object[].class);
        MethodHandle exact = lookup.findStatic(RefTransfer.class, "mixed", type);
        require((Object) exact.invokeExact(object, 7, wide, 3.25, array) == object);
        MethodHandle bound = exact.bindTo(object);
        require((Object) bound.invokeExact(7, wide, 3.25, array) == object);

        RefTransfer holder = new RefTransfer();
        holder.value = object;
        VarHandle field = lookup.findVarHandle(RefTransfer.class, "value", Object.class);
        require(field.compareAndExchange(holder, object, replacement) == object);
        require(holder.value == replacement);
        VarHandle element = MethodHandles.arrayElementVarHandle(Object[].class);
        require(element.getOpaque(array, 0) == object);

        Object[][][] matrix = new Object[2][2][2];
        matrix[1][1][1] = object;
        require(matrix.clone()[1][1][1] == object);
        require(((RefTransfer) holder.clone()).value == replacement);
        require(new String(new char[] {'r', 'o', 'o', 't'}).equals("root"));

        Object reflected = RefTransfer.class.getMethod(
            "mixed", Object.class, int.class, long.class, double.class, Object[].class
        ).invoke(null, object, 7, wide, 3.25, array);
        require(reflected == object);

        RuntimeException thrown = new RuntimeException("handoff");
        MethodHandle raising = lookup.findStatic(
            RefTransfer.class, "raise", MethodType.methodType(void.class, RuntimeException.class));
        try {
            raising.invokeExact(thrown);
            throw new AssertionError("missing MethodHandle exception");
        } catch (RuntimeException caught) {
            require(caught == thrown);
        }
        try {
            RefTransfer.class.getMethod("raise", RuntimeException.class).invoke(null, thrown);
            throw new AssertionError("missing reflection exception");
        } catch (InvocationTargetException caught) {
            require(caught.getCause() == thrown);
        }
        System.out.println("ref-transfer-ok");
    }
}
