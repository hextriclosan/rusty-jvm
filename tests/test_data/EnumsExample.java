package samples.javacore.enums.trivial;

import java.time.format.FormatStyle;
import java.util.Arrays;

public class EnumsExample {
    public static void main(String[] args) {
        print("FormatStyle values", Arrays.toString(FormatStyle.values()));
        FormatStyle medium = FormatStyle.MEDIUM; // fixme: use FormatStyle.valueOf("MEDIUM"); after fixing Native Call Error: Native method jdk/internal/reflect/DirectMethodHandleAccessor$NativeAccessor:invoke0:(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object; not found\n"
        print("FormatStyle.MEDIUM name", medium.name());
        print("FormatStyle.MEDIUM ordinal", medium.ordinal());

        FormatStyle anotherMedium = FormatStyle.MEDIUM;
        print("medium == anotherMedium", medium == anotherMedium);
    }

    private static void print(String description, Object value) {
        System.out.println(description + ": " + value);
    }
}
