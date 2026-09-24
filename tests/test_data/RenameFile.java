package samples.io.renamefile;

import java.io.File;

public class RenameFile {
    public static void main(String[] args) {
        File source = new File(args[0]);
        File destination = new File(args[1]);
        System.out.println(source.renameTo(destination));
        System.out.println(source.exists());
        System.out.println(destination.exists());
    }
}
