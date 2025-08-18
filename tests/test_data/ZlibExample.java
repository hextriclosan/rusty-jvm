package samples.zlib.zlibexample;

import java.io.ByteArrayOutputStream;
import java.nio.charset.StandardCharsets;
import java.util.zip.CRC32;
import java.util.zip.Deflater;
import java.util.zip.Inflater;

public class ZlibExample {
    public static void main(String[] args) throws Exception {
        String original = "banana ".repeat(5_000);
        byte[] input = original.getBytes(StandardCharsets.UTF_8);

        int[] levels = {
                Deflater.DEFAULT_COMPRESSION,
                Deflater.BEST_COMPRESSION
        };
        boolean[] nowrapOptions = {false, true}; // false = normal, true = raw deflate

        CRC32 crc32 = new CRC32();
        crc32.update(input);
        long originalCrc = crc32.getValue();
        System.out.println("Original size: " + input.length + " bytes");
        System.out.println("Original CRC32: " + originalCrc);

        for (boolean nowrap : nowrapOptions) {
            String header = nowrap ? "Raw DEFLATE (nowrap=true)" : "Wrapped DEFLATE";
            System.out.println("=== " + header + " ===");

            for (int level : levels) {
                // Initialize deflater
                Deflater deflater = new Deflater(level, nowrap);
                deflater.setInput(input);
                deflater.finish();

                ByteArrayOutputStream compressedOut = new ByteArrayOutputStream();
                byte[] buffer = new byte[1024];
                while (!deflater.finished()) {
                    int count = deflater.deflate(buffer);
                    compressedOut.write(buffer, 0, count);
                }
                byte[] compressed = compressedOut.toByteArray();
                deflater.end();

                // Decompress
                Inflater inflater = new Inflater(nowrap);
                inflater.setInput(compressed);

                ByteArrayOutputStream decompressedOut = new ByteArrayOutputStream();
                while (!inflater.finished()) {
                    int count = inflater.inflate(buffer);
                    decompressedOut.write(buffer, 0, count);
                }
                inflater.end();

                byte[] decompressed = decompressedOut.toByteArray();

                CRC32 crcCheck = new CRC32();
                crcCheck.update(decompressed);

                System.out.println("Level " + level
                        + ", Compressed size: " + compressed.length
                        + ", Compression ratio: " + (100 * (double) compressed.length / input.length) + "%"
                        + ", Decompressed CRC32: " + crcCheck.getValue()
                        + ", Correct: " + (crcCheck.getValue() == originalCrc));
            }
        }
    }
}
