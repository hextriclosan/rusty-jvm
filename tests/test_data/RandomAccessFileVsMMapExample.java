package samples.io.randomaccessfilevsmmapexample;

import java.io.IOException;
import java.io.RandomAccessFile;
import java.nio.MappedByteBuffer;
import java.nio.channels.FileChannel;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardOpenOption;

public class RandomAccessFileVsMMapExample {

    public static void main(String[] args) throws IOException {
        Path filePath = Paths.get(args[0]);

        writeWithRandomAccessFile(filePath);
        System.out.println();

        modifyWithMemoryMap(filePath);
        System.out.println();

        verifyWithRandomAccessFile(filePath);
        System.out.println();

        // hexDump(filePath);
    }

    private static void writeWithRandomAccessFile(Path filePath) throws IOException {
        try (RandomAccessFile raf = new RandomAccessFile(filePath.toFile(), "rw")) {
            System.out.println("=== RandomAccessFile example ===");

            // Write int, long, double, and string
            raf.seek(0);
            raf.writeInt(0xDEADBEEF);

            raf.seek(8);
            raf.writeLong(0xFEEDFACE01020304L);

            raf.seek(16);
            raf.writeDouble(3.141592653589793);

            String text = "HelloMMap";
            byte[] bytes = text.getBytes(StandardCharsets.UTF_8);
            raf.seek(24);
            raf.writeInt(bytes.length);
            raf.write(bytes);

            // Read back and print
            raf.seek(0);
            int intValue = raf.readInt();
            raf.seek(8);
            long longValue = raf.readLong();
            raf.seek(16);
            double doubleValue = raf.readDouble();
            raf.seek(24);
            String strValue = readString(raf);
            printValues(intValue, longValue, doubleValue, strValue);
        }
    }

    private static void modifyWithMemoryMap(Path filePath) throws IOException {
        try (FileChannel channel = FileChannel.open(filePath, StandardOpenOption.READ, StandardOpenOption.WRITE)) {
            System.out.println("=== Memory-mapped file example ===");

            MappedByteBuffer buffer = channel.map(FileChannel.MapMode.READ_WRITE, 0, 64);

            // Read existing values
            int intValue = buffer.getInt(0);
            long longValue = buffer.getLong(8);
            double doubleValue = buffer.getDouble(16);
            String strValue = readString(buffer, 24);
            printValues(intValue, longValue, doubleValue, strValue);

            // Modify them
            buffer.putInt(0, 0xCAFEBABE);
            buffer.putLong(8, 0xC0DEBA5EDEADBEEFL);
            buffer.putDouble(16, 2.718281828459045);

            String newText = "UpdatedMMap";
            byte[] newBytes = newText.getBytes(StandardCharsets.UTF_8);
            buffer.putInt(24, newBytes.length);
            buffer.put(newBytes);

            buffer.force();
        }
    }

    private static void verifyWithRandomAccessFile(Path filePath) throws IOException {
        try (RandomAccessFile raf = new RandomAccessFile(filePath.toFile(), "r")) {
            System.out.println("=== Verify after mmap modification ===");

            raf.seek(0);
            int intValue = raf.readInt();
            raf.seek(8);
            long longValue = raf.readLong();
            raf.seek(16);
            double doubleValue = raf.readDouble();
            raf.seek(24);
            String strValue = readString(raf);

            printValues(intValue, longValue, doubleValue, strValue);
        }
    }

    // === Utility methods ===
    private static String readString(RandomAccessFile raf) throws IOException {
        int len = raf.readInt();
        byte[] bytes = new byte[len];
        raf.readFully(bytes);
        return new String(bytes, StandardCharsets.UTF_8);
    }

    private static String readString(MappedByteBuffer buffer, int offset) {
        int len = buffer.getInt(offset);
        byte[] bytes = new byte[len];
        buffer.position(offset + 4);
        buffer.get(bytes);
        return new String(bytes, StandardCharsets.UTF_8);
    }

    private static void printValues(int intValue, long longValue, double doubleValue, String strValue) {
        System.out.printf("intValue = 0x%08X%n", intValue);
        System.out.printf("longValue = 0x%016X%n", longValue);
        System.out.printf("doubleValue = %.15f%n", doubleValue);
        System.out.printf("stringValue = %s%n", strValue);
    }

    private static void hexDump(Path path) throws IOException {
        System.out.println("=== Hex Dump ===");
        byte[] bytes = Files.readAllBytes(path);

        for (int i = 0; i < bytes.length; i += 16) {
            System.out.printf("%04X: ", i);
            for (int j = 0; j < 16 && i + j < bytes.length; j++) {
                System.out.printf("%02X ", bytes[i + j]);
            }
            System.out.println();
        }
        System.out.println();
    }
}
