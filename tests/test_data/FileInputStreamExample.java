package samples.io.fileinputstreamexample;

import java.io.FileInputStream;
import java.io.IOException;

public class FileInputStreamExample {
    public static void main(String[] args) throws IOException {
        String path = args[0];
        try (FileInputStream fis = new FileInputStream(path)) {
            System.out.print((char)fis.read());
            System.out.print((char)fis.read());
            System.out.print(new String(fis.readNBytes(2)));
            System.out.print(new String(fis.readAllBytes()));
        }
    }
}
