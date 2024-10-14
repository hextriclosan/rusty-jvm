package samples.javacore.strings.cpool.advanced;

public class StringPoolAdvanced {
    public static void main(String[] args) {
        // Example 0: String literals of same class are stored in the string pool
        String str1 = "Hello";
        String str2 = "Hello";
        // Both str1 and str2 refer to the same string literal from the pool
        int bit0 = str1 == str2 ? 1 : 0;

        // Example 1: Creating a string with new keyword
        String str3 = new String("Hello");
        // str3 is created in the heap, not in the string pool
        int bit1 = str1 == str3 ? 1 : 0;

        // Example 2: Comparing strings with equals method
        int bit2 = str1.equals(str3) ? 1 : 0;

        // Example 3: Concatenation with literals at compile-time
        String str5 = "Hel" + "lo"; // Compiler optimizes this to "Hello"
        int bit3 = str1 == str5 ? 1 : 0;

        // Example 4: Creating a string in another class
        String str4 = new AnotherClass().getAnotherString();
        int bit4 = str1 == str4 ? 1 : 0;

        // Example X: Concatenation with non-literals at runtime
        //String part1 = "Hel";
        //String part2 = "lo";
        //String str6 = part1 + part2; // New object is created in the heap
        //System.out.println("str1 == str6 : " + (str1 == str6)); // false

        // Example X: Interning the string (forcing it into the pool)
        //String str4 = str3.intern();
        // Now str4 refers to the string from the pool
        //System.out.println("str1 == str4 : " + (str1 == str4)); // true

        int result = 0;
        result = setBit(result, 0, bit0);
        result = setBit(result, 1, bit1);
        result = setBit(result, 2, bit2);
        result = setBit(result, 3, bit3);
        result = setBit(result, 4, bit4);
    }

    private static int setBit(int num, int position, int value) {
        return value == 0 ? num & ~(1 << position) : num | (1 << position);
    }
}

class AnotherClass {
    public String getAnotherString() {
        return "Hello";
    }
}
