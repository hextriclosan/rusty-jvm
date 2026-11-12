package samples.jni;

import java.lang.reflect.Field;
import java.lang.reflect.Modifier;

public class JniReflectedFieldsDemo {
    static { System.loadLibrary("jni_test_lib"); }

    private static native Field roundTrip(Field field, Class<?> target, boolean isStatic);
    private static native boolean sameId(Field field, Class<?> target, String name, String signature, boolean isStatic);
    private static native int update(Field field, Object receiver, Class<?> target, int value, boolean isStatic);

    static class Parent {
        public int first;
        public long wide;
        private Object reference;
        public int[] array;
        public static int shared;
    }

    static class Child extends Parent {
        public int first;
        public static int shared;
    }

    public static void main(String[] args) throws Exception {
        Child receiver = new Child();
        for (Class<?> declaring : new Class<?>[] {Parent.class, Child.class}) {
            for (Field field : declaring.getDeclaredFields()) {
                boolean isStatic = Modifier.isStatic(field.getModifiers());
                Field result = roundTrip(field, Child.class, isStatic);
                if (!field.equals(result)) {
                    throw new AssertionError("round trip: " + field);
                }
                if (!sameId(field, declaring, field.getName(), field.getType().descriptorString(), isStatic)) {
                    throw new AssertionError("field ID: " + field);
                }
            }
        }
        Field inherited = Parent.class.getDeclaredField("wide");
        if (!sameId(inherited, Child.class, "wide", "J", false)) {
            throw new AssertionError("inherited ID");
        }
        Field parentFirst = Parent.class.getDeclaredField("first");
        Field childFirst = Child.class.getDeclaredField("first");
        Field parentStatic = Parent.class.getDeclaredField("shared");
        Field childStatic = Child.class.getDeclaredField("shared");
        update(parentFirst, receiver, Parent.class, 11, false);
        update(childFirst, receiver, Child.class, 22, false);
        update(parentStatic, null, Parent.class, 33, true);
        update(childStatic, null, Child.class, 44, true);
        if (((Parent) receiver).first != 11 || receiver.first != 22 || Parent.shared != 33 || Child.shared != 44) {
            throw new AssertionError("field access");
        }
        System.out.println("reflected-fields-ok");
    }
}
