package samples.javacore.strings.cpool.advanced;

public class StringPoolAdvanced {
    public static void main(String[] args) {
        // Example 0: String literals of same class are stored in the string pool
        String str1 = "Hello😂";
        String str2 = "Hello😂";
        // Both str1 and str2 refer to the same string literal from the pool
        if (str1 == str2) {
            System.out.println("str1 and str2 have the same reference");
        }

        // Example 1: Creating a string with new keyword
        String str3 = new String("Hello😂");
        // str3 is created in the heap, not in the string pool
        if (str1 != str3) {
            System.out.println("str1 and str3 have different references");
        }

        // Example 2: Comparing strings with equals method
        if (str1.equals(str3)) {
            System.out.println("str1 and str3 have the same content");
        }

        // Example 3: Concatenation with literals at compile-time
        String str5 = "Hel" + "lo😂"; // Compiler optimizes this to "Hello😂"
        if (str1 == str5) {
            System.out.println("str1 and str5 have the same reference");
        }

        // Example 4: Creating a string in another class
        String str4 = new AnotherClass().getAnotherString();
        if (str1 == str4) {
            System.out.println("str1 and str4 have the same reference");
        }

        // Example 5: Interning the string (forcing it into the pool)
        String str6 = str3.intern();
        if (str1 == str6) {
            System.out.println("str1 and str6 have the same reference");
        }

        // Example 6: Creation in runtime
        String str7 = new String(new char[] {'H', 'e', 'l', 'l', 'o', '\uD83D', '\uDE02'}); // New object is created in the heap
        if (str1 != str7 && str1.equals(str7)) {
            System.out.println("str1 and str7 have different references but the same content");
        }
    }
}

class AnotherClass {
    public String getAnotherString() {
        return "Hello😂";
    }
}
