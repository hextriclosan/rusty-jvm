package samples.io.setfilepermission;

import java.io.File;

public class SetFilePermission {
    public static void main(String[] args) {
        System.out.println(new File(args[0]).setWritable(false, false));
    }
}
