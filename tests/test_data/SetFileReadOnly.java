package samples.io.setfilereadonly;

import java.io.File;

public class SetFileReadOnly {
    public static void main(String[] args) {
        System.out.println(new File(args[0]).setReadOnly());
    }
}
