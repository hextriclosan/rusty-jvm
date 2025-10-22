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

    // === Field Offsets ===
    private static final int OFFSET_INT = 2;
    private static final int OFFSET_LONG = OFFSET_INT + Integer.BYTES + 2;
    private static final int OFFSET_DOUBLE = OFFSET_LONG + Long.BYTES + 2;
    private static final int OFFSET_STRING_LEN = OFFSET_DOUBLE + Double.BYTES + 2;

    public static void main(String[] args) throws IOException {
        Path filePath = Paths.get(args[0]);

        writeWithRandomAccessFile(filePath);
        System.out.println();

        modifyWithMemoryMap(filePath);
        System.out.println();

        verifyWithRandomAccessFile(filePath);
        System.out.println();

        hexDump(filePath);
    }

    // === 1. Write initial data using RandomAccessFile ===
    private static void writeWithRandomAccessFile(Path filePath) throws IOException {
        try (RandomAccessFile raf = new RandomAccessFile(filePath.toFile(), "rw")) {
            System.out.println("=== RandomAccessFile example ===");

            // Write int, long, double, and string
            raf.seek(OFFSET_INT);
            raf.writeInt(0xDEADBEEF);

            raf.seek(OFFSET_LONG);
            raf.writeLong(0xFEEDFACE01020304L);

            raf.seek(OFFSET_DOUBLE);
            raf.writeDouble(3.141592653589793);

            writeString(raf, OFFSET_STRING_LEN, "HelloMMap");

            // Read back and print
            raf.seek(OFFSET_INT);
            int intValue = raf.readInt();
            raf.seek(OFFSET_LONG);
            long longValue = raf.readLong();
            raf.seek(OFFSET_DOUBLE);
            double doubleValue = raf.readDouble();
            String strValue = readString(raf, OFFSET_STRING_LEN);
            printValues(intValue, longValue, doubleValue, strValue);
        }
    }

    // === 2. Modify data using memory-mapped file ===
    private static void modifyWithMemoryMap(Path filePath) throws IOException {
        try (FileChannel channel = FileChannel.open(filePath, StandardOpenOption.READ, StandardOpenOption.WRITE)) {
            System.out.println("=== Memory-mapped file example ===");

            MappedByteBuffer buffer = channel.map(FileChannel.MapMode.READ_WRITE, 0, 64);

            // Read existing values
            int intValue = buffer.getInt(OFFSET_INT);
            long longValue = buffer.getLong(OFFSET_LONG);
            double doubleValue = buffer.getDouble(OFFSET_DOUBLE);
            String strValue = readString(buffer, OFFSET_STRING_LEN);
            printValues(intValue, longValue, doubleValue, strValue);

            // Modify them
            buffer.putInt(OFFSET_INT, 0xCAFEBABE);
            buffer.putLong(OFFSET_LONG, 0xC0DEBA5EDEADBEEFL);
            buffer.putDouble(OFFSET_DOUBLE, 2.718281828459045);

            String newText = "Water boils at 100 °C";
            writeString(buffer, OFFSET_STRING_LEN, newText);

            buffer.force();
        }
    }

    // === 3. Verify results using RandomAccessFile ===
    private static void verifyWithRandomAccessFile(Path filePath) throws IOException {
        try (RandomAccessFile raf = new RandomAccessFile(filePath.toFile(), "r")) {
            System.out.println("=== Verify after mmap modification ===");

            raf.seek(OFFSET_INT);
            int intValue = raf.readInt();
            raf.seek(OFFSET_LONG);
            long longValue = raf.readLong();
            raf.seek(OFFSET_DOUBLE);
            double doubleValue = raf.readDouble();
            String strValue = readString(raf, OFFSET_STRING_LEN);

            printValues(intValue, longValue, doubleValue, strValue);
        }
    }

    // === Utility methods ===
    private static String readString(RandomAccessFile raf, int offset) throws IOException {
        raf.seek(offset);
        int len = raf.readInt();
        byte[] bytes = new byte[len];
        raf.readFully(bytes);
        return new String(bytes, StandardCharsets.UTF_8);
    }

    private static void writeString(RandomAccessFile raf, int offset, String value) throws IOException {
        byte[] bytes = value.getBytes(StandardCharsets.UTF_8);
        raf.seek(offset);
        raf.writeInt(bytes.length);
        raf.write(bytes);
    }

    private static String readString(MappedByteBuffer buffer, int offset) {
        buffer.position(offset);
        int len = buffer.getInt();
        byte[] bytes = new byte[len];
        buffer.get(bytes);
        return new String(bytes, StandardCharsets.UTF_8);
    }

    private static void writeString(MappedByteBuffer buffer, int offset, String value) {
        byte[] bytes = value.getBytes(StandardCharsets.UTF_8);
        buffer.position(offset);
        buffer.putInt(bytes.length);
        buffer.put(bytes);
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
            // Print offset
            System.out.printf("%04X:", i);

            // Print hex bytes
            for (int j = 0; j < 16; j++) {
                if (i + j < bytes.length) {
                    System.out.printf(" %02X", bytes[i + j] & 0xFF);
                } else {
                    System.out.print("   "); // padding for short last line
                }
            }

            // Space between hex and ASCII
            System.out.print("  ");

            // Print ASCII representation
            for (int j = 0; j < 16 && i + j < bytes.length; j++) {
                int b = bytes[i + j] & 0xFF;
                char c = (b >= 32 && b <= 126) ? (char) b : '.';
                System.out.print(c);
            }

            System.out.println();
        }
    }
}
