package samples.reflection.trivial.synthetic.classes;

public class SyntheticPrimitiveClasses {
    public static void main(String[] args) {
        Class<?> intClass = int.class;
        Class<?> doubleClass = double.class;
        Class<?> booleanClass = boolean.class;
        Class<?> charClass = char.class;
        Class<?> byteClass = byte.class;
        Class<?> shortClass = short.class;
        Class<?> longClass = long.class;
        Class<?> floatClass = float.class;
        Class<?> voidClass = void.class;

        int modifiersInt = intClass.getModifiers();
        int modifiersDouble = doubleClass.getModifiers();
        int modifiersBoolean = booleanClass.getModifiers();
        int modifiersChar = charClass.getModifiers();
        int modifiersByte = byteClass.getModifiers();
        int modifiersShort = shortClass.getModifiers();
        int modifiersLong = longClass.getModifiers();
        int modifiersFloat = floatClass.getModifiers();
        int modifiersVoid = voidClass.getModifiers();

        int modifiersSum = modifiersInt + modifiersDouble + modifiersBoolean + modifiersChar + modifiersByte + modifiersShort + modifiersLong + modifiersFloat + modifiersVoid;
    }
}
