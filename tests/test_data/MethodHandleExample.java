package samples.reflection.methodhandleexample;

import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.invoke.VarHandle;

import java.util.ArrayList;

/** MethodHandleExample demonstrates a full range of MethodHandles capabilities, including:
 * - findStatic – Static method invocation (Math.pow).
 * - findVirtual – Instance method invocation (String.toUpperCase).
 * - findSpecial – Invoking superclass methods (Parent.testMethod).
 * - findConstructor – Invoking constructors (ArrayList, StringBuilder(String)).
 * - findGetter / findSetter – Accessing instance fields.
 * - findStaticGetter / findStaticSetter – Accessing static fields.
 * - findVarHandle – Mutable instance field manipulation via VarHandle.
 * - findStaticVarHandle – Mutable static field manipulation via VarHandle.
 * - bindTo – Binding a method handle to a specific instance.
 */
public class MethodHandleExample {

    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        System.out.printf("MethodHandles Lookup: %s%n", lookup);

        demonstrateFindStatic(lookup);
        demonstrateFindVirtual(lookup);
        demonstrateFindSpecial(lookup);
        demonstrateFindConstructorWithoutArgs(lookup);
        demonstrateFindConstructorWithArgs(lookup);
        demonstrateFindGetterSetter(lookup);
        //demonstrateFindStaticGetterSetter(lookup); // Execution Error: error getting instance field value java/lang/invoke/MethodType.rtype
        //demonstrateFindVarHandle(lookup); // Execution Error: error getting instance field value java/lang/invoke/MethodType.rtype
        //demonstrateFindStaticVarHandle(lookup); // Execution Error: error getting instance field value java/lang/invoke/MethodType.rtype
        //demonstrateBindTo(lookup); // Native method java/lang/Class:isInstance:(Ljava/lang/Object;)Z not found
    }

    private static void demonstrateFindStatic(MethodHandles.Lookup lookup) throws Throwable {
        System.out.println("------- findStatic (Math.pow) -------");
        MethodType methodType = MethodType.methodType(double.class, double.class, double.class);
        MethodHandle mh = lookup.findStatic(Math.class, "pow", methodType);
        sampleMethod(methodType, mh, (double) mh.invokeExact(2.0, 3.0));
    }

    private static void demonstrateFindVirtual(MethodHandles.Lookup lookup) throws Throwable {
        System.out.println("------- findVirtual (String.regionMatches) -------");
        MethodType methodType = MethodType.methodType(boolean.class, boolean.class, int.class, String.class, int.class, int.class);
        MethodHandle mh = lookup.findVirtual(String.class, "regionMatches", methodType);
        String map = "X marks the spot where the legendary gold doubloons are buried!";
        String gold = "gold";
        boolean ignoreCase = true;
        boolean result = (boolean) mh.invokeExact(map, true, map.indexOf(gold), gold, 0, gold.length());
        sampleMethod(methodType, mh, Boolean.toString(result));
    }

    private static void demonstrateFindSpecial(MethodHandles.Lookup lookup) throws Throwable {
        System.out.println("------- findSpecial (Parent.testMethod) -------");
        Child instance = new Child();
        MethodHandle parentMh = instance.getParentMethodHandle();
        String result = (String) parentMh.invokeExact(instance);
        sampleMethod(MethodType.methodType(String.class), parentMh, result);
    }

    private static void demonstrateFindConstructorWithoutArgs(MethodHandles.Lookup lookup) throws Throwable {
        System.out.println("------- findConstructor (ArrayList) -------");
        MethodType constructorType = MethodType.methodType(void.class);
        MethodHandle constructor = lookup.findConstructor(ArrayList.class, constructorType);
        ArrayList<Integer> list = (ArrayList<Integer>) constructor.invokeExact();
        list.add(1337);
        sampleMethod(constructorType, constructor, list);
    }

    private static void demonstrateFindConstructorWithArgs(MethodHandles.Lookup lookup) throws Throwable {
        System.out.println("------- findConstructor (StringBuilder(String)) -------");
        MethodType constructorType = MethodType.methodType(void.class, String.class);
        MethodHandle constructor = lookup.findConstructor(StringBuilder.class, constructorType);
        String initialValue = "1 + ";
        StringBuilder stringBuilder = (StringBuilder) constructor.invokeExact(initialValue);
        stringBuilder.append(1);
        stringBuilder.append(" = 2");
        sampleMethod(constructorType, constructor, stringBuilder);
    }

    private static void demonstrateFindGetterSetter(MethodHandles.Lookup lookup) throws Throwable {
        System.out.println("------- findGetter / findSetter (SampleClass.value) -------");
        MethodHandle getter = lookup.findGetter(SampleClass.class, "value", int.class);
        MethodHandle setter = lookup.findSetter(SampleClass.class, "value", int.class);

        SampleClass obj = new SampleClass();
        setter.invokeExact(obj, 42);
        int value = (int) getter.invokeExact(obj);
        sampleMethod(MethodType.methodType(int.class, SampleClass.class), getter, value);
    }

    private static void demonstrateFindStaticGetterSetter(MethodHandles.Lookup lookup) throws Throwable {
        System.out.println("------- findStaticGetter / findStaticSetter (staticValue) -------");
        MethodHandle getter = lookup.findStaticGetter(SampleClass.class, "staticValue", int.class);
        MethodHandle setter = lookup.findStaticSetter(SampleClass.class, "staticValue", int.class);

        setter.invokeExact(500);
        int value = (int) getter.invokeExact();
        sampleMethod(MethodType.methodType(int.class), getter, value);
    }

    private static void demonstrateFindVarHandle(MethodHandles.Lookup lookup) throws Throwable {
        System.out.println("------- findVarHandle (SampleClass.value) -------");
        VarHandle varHandle = lookup.findVarHandle(SampleClass.class, "value", int.class);

        SampleClass obj = new SampleClass();
        varHandle.set(obj, 99);
        int value = (int) varHandle.get(obj);
        sampleMethod(MethodType.methodType(int.class, SampleClass.class), null, value);
    }

    private static void demonstrateFindStaticVarHandle(MethodHandles.Lookup lookup) throws Throwable {
        System.out.println("------- findStaticVarHandle (staticValue) -------");
        VarHandle varHandle = lookup.findStaticVarHandle(SampleClass.class, "staticValue", int.class);

        varHandle.set(1000);
        int value = (int) varHandle.get();
        sampleMethod(MethodType.methodType(int.class), null, value);
    }

    private static void demonstrateBindTo(MethodHandles.Lookup lookup) throws Throwable {
        System.out.println("------- bindTo (String.length) -------");
        MethodType methodType = MethodType.methodType(int.class);
        MethodHandle mh = lookup.findVirtual(String.class, "length", methodType);
        MethodHandle boundMh = mh.bindTo("Invoke String.length on me");
        int length = (int) boundMh.invokeExact();
        sampleMethod(methodType, boundMh, length);
    }

    private static void sampleMethod(MethodType methodType, MethodHandle mh, Object result) {
        System.out.printf("%s - %s: %s%n", methodType, mh, result);
    }
}

class SampleClass {
    public static int staticValue = 100;
    public int value;
}

class Parent {
    public String testMethod() {
        return "Parent method invoked";
    }
}

class Child extends Parent {
    /**
     * We need to create a lookup from within Child itself, as it has direct access to its superclass's methods.
     * Otherwise, we would get an IllegalAccessException because findSpecial requires MethodHandles.Lookup
     * to have special privileges to invoke testMethod() in Parent from Child.
     * By default, MethodHandles.lookup() does not provide private or special access across different classes unless explicitly granted.
     */
    public MethodHandle getParentMethodHandle() throws NoSuchMethodException, IllegalAccessException {
        return MethodHandles.lookup().findSpecial(Parent.class, "testMethod", MethodType.methodType(String.class), Child.class);
    }

    @Override
    public String testMethod() {
        return "Child method invoked";
    }
}
