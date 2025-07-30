package samples.reflection.trivial.isenumexample;

import java.util.concurrent.TimeUnit;

public class IsEnumExample {
    public static void main(String[] args) {
        // 1. Basic Case: Enum class
        print(TimeUnit.class); // true

        // 2. Case: Enum constant's class
        print(TimeUnit.MINUTES.getClass()); // true

        // 3. Case: Non-enum class
        print(String.class); // false

        // 4. Case: Anonymous subclass of Enum (simulated with reflection)
//        Enum<?> anonymousEnum = Enum.valueOf(TimeUnit.class, "MINUTES"); // Native Call Error: Native method jdk/internal/reflect/DirectMethodHandleAccessor$NativeAccessor:invoke0:(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object; not found
//        print(anonymousEnum.getClass()); // true

        // 5. Case: Array of enums
        print(TimeUnit[].class); // false

        // 6. Case: Interface
        print(Runnable.class); // false

        // 7. Case: Primitive type
        print(int.class); // false

        // 8. Case: Void type
        print(void.class); // false
    }

    private static void print(Class<?> clazz) {
        System.out.println("Is " + clazz.getSimpleName() + " enum: " + clazz.isEnum());
    }
}
