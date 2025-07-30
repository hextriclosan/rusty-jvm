package samples.javacore.enums.trivial;

import java.time.format.FormatStyle;
import java.util.Arrays;

public class EnumsExample {
    public static void main(String[] args) {
        print("FormatStyle values", Arrays.toString(FormatStyle.values()));
        FormatStyle medium = FormatStyle.valueOf("MEDIUM");
        print("FormatStyle.MEDIUM name", medium.name());
        print("FormatStyle.MEDIUM ordinal", medium.ordinal());

        FormatStyle anotherMedium = FormatStyle.MEDIUM;
        print("medium == anotherMedium", medium == anotherMedium);
    }

    private static void print(String description, Object value) {
        System.out.println(description + ": " + value);
    }
}
