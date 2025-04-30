// javac -XDstringConcat=inline  -d . FileDispatcherExample.java
package samples.nio.filedispatcherexample;

import java.io.IOException;
import java.nio.ByteBuffer;
import java.nio.channels.FileChannel;
import java.nio.file.Path;
import java.nio.file.StandardOpenOption;

public class FileDispatcherExample {

    public static void main(String[] args) throws IOException {
        Path path = Path.of("../tmp/file_dispatcher_example.txt");

        // Write to the file (uses FileDispatcherImpl.write0 under the hood)
        try (FileChannel channel = FileChannel.open(path, StandardOpenOption.CREATE, StandardOpenOption.WRITE)) {
            ByteBuffer buffer = ByteBuffer.wrap("Hello from FileChannel!".getBytes());
            channel.write(buffer); // <- This uses FileDispatcherImpl.write0
        }

        // Read from the file (uses FileDispatcherImpl.read0 internally)
//         try (FileChannel channel = FileChannel.open(path, StandardOpenOption.READ)) {
//             ByteBuffer buffer = ByteBuffer.allocate(64);
//             int bytesRead = channel.read(buffer); // <- This uses FileDispatcherImpl.read0
//             buffer.flip();
//
//             byte[] data = new byte[bytesRead];
//             buffer.get(data);
//             System.out.println("Read: " + new String(data));
//         }
    }
}
