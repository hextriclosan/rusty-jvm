package samples.reflection.trivial.isenumexample;

import java.util.concurrent.TimeUnit;

public class IsEnumExample {
    public static void main(String[] args) {
        // 1. Basic Case: Enum class
        System.out.print("Is TimeUnit enum: ");
        System.out.println(TimeUnit.class.isEnum()); // true

        // 2. Case: Enum constant's class
        System.out.print("Is TimeUnit.MINUTES enum: ");
        System.out.println(TimeUnit.MINUTES.getClass().isEnum()); // true

        // 3. Case: Non-enum class
        System.out.print("Is String enum: ");
        System.out.println(String.class.isEnum()); // false

        // 4. Case: Anonymous subclass of Enum (simulated with reflection)
//         Enum<?> anonymousEnum = Enum.valueOf(TimeUnit.class, "MINUTES"); // Native Call Error: Native method jdk/internal/reflect/DirectMethodHandleAccessor$NativeAccessor:invoke0:(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object; not found
//         System.out.print("Is Anonymous Enum enum: ");
//         System.out.println(anonymousEnum.getClass().isEnum()); // true

        // 5. Case: Array of enums
        System.out.print("Is TimeUnit[].class enum: ");
        System.out.println(TimeUnit[].class.isEnum()); // false

        // 6. Case: Interface
        System.out.print("Is Runnable enum: ");
        System.out.println(Runnable.class.isEnum()); // false

        // 7. Case: Primitive type
        System.out.print("Is int enum: ");
        System.out.println(int.class.isEnum()); // false

        // 8. Case: Void type
        System.out.print("Is void enum: ");
        System.out.println(void.class.isEnum()); // false
    }
}
