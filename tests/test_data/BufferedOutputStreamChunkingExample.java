package samples.io.bufferedoutputstreamchunkingexample;

import java.io.BufferedOutputStream;
import java.io.FileOutputStream;
import java.io.IOException;

public class BufferedOutputStreamChunkingExample {
    public static void main(String[] args) throws IOException {
        try (BufferedOutputStream bos = new BufferedOutputStream(
                new FileOutputStream("../tmp/buffered_output.txt"), 8)) { // Set buffer size to 8 bytes
            byte[] data = "This is a test for BufferedOutputStream chunking.".getBytes();

            // Write data byte by byte
            for (byte b : data) {
                bos.write(b);
            }

            bos.flush(); // Flushing remaining data.
        }
    }
}
