package samples.io.randomaccessfilevsmmapexample;

import java.io.IOException;
import java.io.RandomAccessFile;
import java.nio.MappedByteBuffer;
import java.nio.channels.FileChannel;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardOpenOption;

public class RandomAccessFileVsMMapExample {

    public static void main(String[] args) throws IOException {
        String path = args[0];
        Path filePath = Paths.get(path);

        // === CASE 1: Regular RandomAccessFile I/O ===
        try (RandomAccessFile raf = new RandomAccessFile(filePath.toFile(), "rw")) {
            System.out.println("=== RandomAccessFile example ===");

            // Write primitive values at specific positions
            raf.seek(0);
            raf.writeInt(0xDEADBEEF);

            raf.seek(8);
            raf.writeLong(0xFEEDFACE01020304L);

            raf.seek(0);
            int intValue = raf.readInt();

            raf.seek(8);
            long longValue = raf.readLong();

            System.out.printf("intValue = 0x%08X%n", intValue);
            System.out.printf("longValue = 0x%016X%n", longValue);
        }
        System.out.println();

        // === CASE 2: Memory-mapped I/O ===
        try (FileChannel channel = FileChannel.open(filePath, StandardOpenOption.READ, StandardOpenOption.WRITE)) {
            System.out.println("=== Memory-mapped file example ===");

            // Map first 16 bytes of the file
            MappedByteBuffer buffer = channel.map(FileChannel.MapMode.READ_WRITE, 0, 16);

            // Read what was written before
            int intValue = buffer.getInt(0);
            long longValue = buffer.getLong(8);
            System.out.printf("Read via mmap -> intValue = 0x%08X, longValue = 0x%016X%n", intValue, longValue);

            // Modify file directly via memory
            buffer.putInt(0, 0xCAFEBABE);
            buffer.putLong(8, 0xC0DEBA5EDEADBEEFL);

            // Force changes to disk
            buffer.force();
        }
        System.out.println();

        // === Verify modifications via regular I/O again ===
        try (RandomAccessFile raf = new RandomAccessFile(filePath.toFile(), "r")) {
            System.out.println("=== Verify after mmap modification ===");

            raf.seek(0);
            int intValue = raf.readInt();

            raf.seek(8);
            long longValue = raf.readLong();

            System.out.printf("After mmap -> intValue = 0x%08X, longValue = 0x%016X%n", intValue, longValue);
        }
    }
}
