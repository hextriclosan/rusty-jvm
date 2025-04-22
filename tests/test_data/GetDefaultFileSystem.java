// javac -XDstringConcat=inline  -d . GetDefaultFileSystem.java
package samples.filesystem.getdefaultfilesystem;

import java.nio.file.FileSystem;
import java.nio.file.FileSystems;
import java.nio.file.Path;
import java.nio.file.spi.FileSystemProvider;
import java.util.Set;
import java.util.TreeSet;

public class GetDefaultFileSystem {
    public static void main(String[] args) {
        FileSystem fileSystem = FileSystems.getDefault();
        FileSystemProvider provider = fileSystem.provider();

        System.out.println("Default FileSystem class: " + fileSystem.getClass().getSimpleName());
        System.out.println("FileSystem provider: " + provider.getClass().getName());
        System.out.println("Separator: " + fileSystem.getSeparator());
        System.out.println("Supported file attribute views:" + new TreeSet<>(fileSystem.supportedFileAttributeViews()));
        System.out.println("Is open: " + fileSystem.isOpen());
        System.out.println("Is read-only: " + fileSystem.isReadOnly());
    }
}
