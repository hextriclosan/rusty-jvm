package samples.io.setfilelastmodified;

import java.io.File;

public class SetFileLastModified {
    public static void main(String[] args) {
        System.out.println(new File(args[0]).setLastModified(Long.parseLong(args[1])));
    }
}
