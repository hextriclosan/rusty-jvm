package samples.nio.filetypechecks;

import java.io.IOException;
import java.nio.file.FileSystemException;
import java.nio.file.Files;
import java.nio.file.LinkOption;
import java.nio.file.Path;
import java.nio.file.attribute.BasicFileAttributes;

// On JDK 26+ the followed-link checks below stat through `UnixNativeDispatcher.fstatat0`, which
// reports a missing file (ENOENT) or a non-directory path prefix (ENOTDIR) as an errno instead of
// throwing.
public class FileTypeChecksExample {
    public static void main(String[] args) throws IOException {
        if (args.length < 1) {
            System.err.println("Usage: java FileTypeChecksExample <dir>");
            return;
        }

        Path dir = Path.of(args[0]);
        Path file = dir.resolve("file.txt");
        Path subdir = dir.resolve("dir");
        Path link = dir.resolve("link");

        System.out.println("file.txt is regular file: " + Files.isRegularFile(file));
        System.out.println("file.txt is directory: " + Files.isDirectory(file));
        System.out.println("dir is directory: " + Files.isDirectory(subdir));
        System.out.println("dir is regular file: " + Files.isRegularFile(subdir));
        System.out.println("link is regular file: " + Files.isRegularFile(link));
        System.out.println("link is regular file (nofollow): " + Files.isRegularFile(link, LinkOption.NOFOLLOW_LINKS));
        System.out.println("missing.txt is regular file: " + Files.isRegularFile(dir.resolve("missing.txt")));
        System.out.println("file.txt/child is directory: " + Files.isDirectory(file.resolve("child")));
        System.out.println("link is same file as file.txt: " + Files.isSameFile(link, file));
        System.out.println("dir is same file as file.txt: " + Files.isSameFile(subdir, file));

        // an errno without a dedicated exception (ENAMETOOLONG) is reported with its `strerror` message
        try {
            Files.readAttributes(dir.resolve("x".repeat(300)), BasicFileAttributes.class);
        } catch (FileSystemException e) {
            System.out.println("too long name attributes: " + e.getClass().getName() + ": " + e.getReason());
        }
    }
}
