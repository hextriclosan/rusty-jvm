package samples.nio.filedispatcherimpl.filedispatcherimpldemo;

import java.io.InputStream;
import java.nio.channels.Channels;
import java.nio.channels.FileChannel;
import java.nio.file.StandardOpenOption;
import java.io.File;
import java.io.FileWriter;

public class FileDispatcherImplDemo {
    public static void main(String[] args) throws Exception {
        // 1. Create temp file
        File tempFile = File.createTempFile("input_stream_demo", ".txt");
        tempFile.deleteOnExit();

        // 2. Write string to the file
        try (FileWriter writer = new FileWriter(tempFile)) {
            writer.write("Hello, this is a test file for InputStream and FileChannel.");
        }

        // 3. Read using ChannelInputStream: This will internally use isOther0 and seek0 native methods of UnixFileDispatcherImpl or FileDispatcherImpl
        try (FileChannel channel = FileChannel.open(tempFile.toPath(), StandardOpenOption.READ);
             InputStream in = Channels.newInputStream(channel)) {
            byte[] bytes = in.readAllBytes();
            System.out.println(new String(bytes));
        }
    }
}
