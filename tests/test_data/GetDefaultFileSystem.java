package samples.filesystem.getdefaultfilesystem;

import java.nio.file.FileSystems;

public class GetDefaultFileSystem {
    public static void main(String[] args) {
        System.out.println(FileSystems.getDefault().getClass().getSimpleName());
    }
}
