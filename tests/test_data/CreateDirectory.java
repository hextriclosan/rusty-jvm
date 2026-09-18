package samples.io.createdirectory;

import java.io.File;

public class CreateDirectory {
    public static void main(String[] args) {
        File directory = new File(args[0]);
        System.out.println(directory.mkdir());
        System.out.println(directory.isDirectory());
        System.out.println(directory.mkdir());
    }
}
