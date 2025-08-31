package samples.io.fileoutputstreamexample;

import java.io.FileOutputStream;
import java.io.IOException;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;

public class FileOutputStreamExample {
    static private final boolean APPEND = true;

    public static void main(String[] args) throws IOException {
        String fileName = args[0];
        try (OutputStream fos = new FileOutputStream(fileName)) {
            fos.write('C');
            fos.write('A');
        }
        try (OutputStream fos = new FileOutputStream(fileName, APPEND)) {
            fos.write("FE".getBytes(StandardCharsets.UTF_8));
        }
        try (OutputStream fos = new FileOutputStream(fileName, APPEND)) {
            fos.write("BA".getBytes());
        }
        try (OutputStream fos = new FileOutputStream(fileName, APPEND)) {
            fos.write("TO BE OR NOT TO BE".getBytes(), 3, 2);
        }
    }
}
