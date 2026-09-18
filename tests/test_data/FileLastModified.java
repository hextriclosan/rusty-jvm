package samples.io.filelastmodified;

import java.io.File;

public class FileLastModified {
    public static void main(String[] args) {
        System.out.println(new File(args[0]).lastModified());
    }
}
