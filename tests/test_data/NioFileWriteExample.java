// javac -XDstringConcat=inline  -d . NioFileWriteExample.java
package samples.nio.niowritefileexample;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardOpenOption;
import java.util.List;

public class NioFileWriteExample {
    public static void main(String[] args) throws IOException {
        Path path = Paths.get("../tmp/write_nio_test.txt");

        // Content to write
        List<String> lines = List.of(
                "Hello, world!",
                "This is written using NIO."
        );

        // Create or overwrite the file and write the content
        Files.write(path, lines, StandardCharsets.UTF_8, StandardOpenOption.CREATE, StandardOpenOption.TRUNCATE_EXISTING);
    }
}
