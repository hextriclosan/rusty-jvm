package samples.io.printstreamexample;

import java.io.FileOutputStream;
import java.io.IOException;
import java.io.PrintStream;

public class PrintStreamExample {
    private static final String FILE_NAME = "../tmp/print_stream_test.txt";
    private static final boolean APPEND = true;

    public static void main(String[] args) throws IOException {
        try (PrintStream ps = new PrintStream(FILE_NAME)) {
            ps.println("Hello, PrintStream!");
        }

        try (PrintStream ps = new PrintStream(new FileOutputStream(FILE_NAME, APPEND))) {
            ps.println("First Line");
            ps.println("Second Line");
            ps.println("Third Line");
        }

//      Exception in thread \"system\" java.util.MissingResourceException: Can\'t find bundle for base name sun.text.resources.FormatData, locale format.language_value_FORMAT.COUNTRY_VALUE_format.variant_VALUE_#Format.script_value\r
//         try (PrintStream ps = new PrintStream(new FileOutputStream(FILE_NAME, APPEND))) {
//             ps.printf("Hello %s, you are %d years old.%n", "Alice", 30);
//         }

        try (PrintStream ps = new PrintStream(new FileOutputStream(FILE_NAME, APPEND))) {
            System.setOut(ps); // Redirects standard output to the file
            System.out.println("This will go to the file instead of the console.");
        }

        try (PrintStream ps = new PrintStream(new FileOutputStream(FILE_NAME, APPEND))) {
            ps.write("Hello as raw bytes".getBytes()); // Writes raw bytes
            ps.println();
        }

//      Exception in thread \"system\" java.util.MissingResourceException: Can\'t find bundle for base name sun.text.resources.FormatData, locale format.language_value_FORMAT.COUNTRY_VALUE_format.variant_VALUE_#Format.script_value\r
//         try (PrintStream ps = new PrintStream(new FileOutputStream(FILE_NAME, APPEND))) {
//             ps.println(new Person("John", 25));
//         }

        try (PrintStream ps = new PrintStream(new FileOutputStream(FILE_NAME, APPEND))) {
            ps.print("This is written immediately. ");
            ps.flush(); // Ensures the data is written to the file
            ps.println("This follows after flush.");
        }

        try (PrintStream ps = new PrintStream(new PrintStream(new FileOutputStream(FILE_NAME, APPEND)))) {
            ps.println("This is an example of chaining PrintStreams.");
        }
    }
}

class Person {
    private final String name;
    private final int age;

    public Person(String name, int age) {
        this.name = name;
        this.age = age;
    }

    @Override
    public String toString() {
        return String.format("Person{name='%s', age=%d}", name, age);
    }
}
