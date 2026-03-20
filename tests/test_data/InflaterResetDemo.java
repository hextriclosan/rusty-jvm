package samples.zlib.inflaterresetdemo;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.zip.DataFormatException;
import java.util.zip.Deflater;
import java.util.zip.Inflater;

public class InflaterResetDemo {
    public static void main(String[] args) throws DataFormatException, IOException {
        byte[] data1 = "AAAAA".repeat(1000).getBytes(StandardCharsets.UTF_8);
        byte[] data2 = "BBBBB".repeat(1000).getBytes(StandardCharsets.UTF_8);

        byte[] compressed1 = compress(data1);
        byte[] compressed2 = compress(data2);

        try (Inflater inflater = new Inflater(); ByteArrayOutputStream out = new ByteArrayOutputStream()) {
            byte[] buffer = new byte[16];
            // --- FIRST STREAM (intentionally NOT fully consumed) ---
            inflater.setInput(compressed1);

            // Only inflate once -> leave the stream unfinished
            inflater.inflate(buffer);
            System.out.println("Read from first stream: " + new String(buffer, StandardCharsets.UTF_8));
            System.out.println("First stream partially processed");

            // --- SECOND STREAM (reuse with reset()) ---
            inflater.reset();
            inflater.setInput(compressed2);

            while (!inflater.finished()) {
                int n = inflater.inflate(buffer);
                if (n == 0) break; // avoid infinite loop
                out.write(buffer, 0, n);
            }

            System.out.println("Second stream result size: " + out.size());
        }
    }

    private static byte[] compress(byte[] input) {
        try(Deflater deflater = new Deflater()) {
            deflater.setInput(input);
            deflater.finish();

            ByteArrayOutputStream out = new ByteArrayOutputStream();
            byte[] buf = new byte[128];

            while (!deflater.finished()) {
                int n = deflater.deflate(buf);
                out.write(buf, 0, n);
            }

            return out.toByteArray();
        }
    }
}
