package samples.io.fileoutputstreamexample;

import java.io.FileOutputStream;
import java.io.IOException;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;

public class FileOutputStreamExample {
    static private final String FILE_NAME = "../tmp/test.txt";
    static private final boolean APPEND = true;

    public static void main(String[] args) throws IOException {
        try (OutputStream fos = new FileOutputStream(FILE_NAME)) {
            fos.write('C');
            fos.write('A');
        }
        try (OutputStream fos = new FileOutputStream(FILE_NAME, APPEND)) {
            fos.write("FE".getBytes(StandardCharsets.UTF_8));
        }
        try (OutputStream fos = new FileOutputStream(FILE_NAME, APPEND)) {
            fos.write("BA".getBytes());
        }
        try (OutputStream fos = new FileOutputStream(FILE_NAME, APPEND)) {
            fos.write("TO BE OR NOT TO BE".getBytes(), 3, 2);
        }
    }
}
