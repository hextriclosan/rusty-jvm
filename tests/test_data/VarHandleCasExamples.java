package samples.concurrency.varhandle;

import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;

/**
 * Directly exercises {@link VarHandle} {@code compareAndSet} / {@code compareAndExchange} /
 * {@code weakCompareAndSet} for {@code int} and reference instance fields (the field-instance CAS
 * branches the atomics test only reaches indirectly).
 *
 * <p>Boolean fields are intentionally excluded: the JDK routes {@code compareAndSetBoolean} through
 * word-masked {@code compareAndSetByte}, which is incompatible with this VM's slot-index field
 * offsets, so boolean-field CAS is not reliable here.
 */
public class VarHandleCasExamples {
    int i = 10;
    String s = "a";

    private static final VarHandle I;
    private static final VarHandle S;

    static {
        try {
            MethodHandles.Lookup lookup = MethodHandles.lookup();
            I = lookup.findVarHandle(VarHandleCasExamples.class, "i", int.class);
            S = lookup.findVarHandle(VarHandleCasExamples.class, "s", String.class);
        } catch (ReflectiveOperationException e) {
            throw new ExceptionInInitializerError(e);
        }
    }

    public static void main(String[] args) {
        VarHandleCasExamples o = new VarHandleCasExamples();

        System.out.println("int compareAndSet ok = " + I.compareAndSet(o, 10, 20));
        System.out.println("int compareAndExchange witness = " + (int) I.compareAndExchange(o, 20, 30));
        System.out.println("int compareAndExchange fail witness = " + (int) I.compareAndExchange(o, 999, 40));
        System.out.println("int compareAndExchangeAcquire witness = " + (int) I.compareAndExchangeAcquire(o, 30, 40));
        System.out.println("int compareAndExchangeRelease witness = " + (int) I.compareAndExchangeRelease(o, 40, 50));
        System.out.println("int weakCompareAndSet = " + I.weakCompareAndSet(o, 50, 60));
        System.out.println("int weakCompareAndSetPlain = " + I.weakCompareAndSetPlain(o, 60, 70));
        System.out.println("int weakCompareAndSetAcquire = " + I.weakCompareAndSetAcquire(o, 70, 80));
        System.out.println("int weakCompareAndSetRelease = " + I.weakCompareAndSetRelease(o, 80, 90));
        System.out.println("int value = " + o.i);

        System.out.println("ref compareAndSet ok = " + S.compareAndSet(o, "a", "b"));
        System.out.println("ref compareAndExchange witness = " + (String) S.compareAndExchange(o, "b", "c"));
        System.out.println("ref compareAndExchange fail witness = " + (String) S.compareAndExchange(o, "zzz", "d"));
        System.out.println("ref weakCompareAndSet = " + S.weakCompareAndSet(o, "c", "d"));
        System.out.println("ref value = " + o.s);
    }
}
