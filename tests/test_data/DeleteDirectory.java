package samples.io.deletedirectory;

import java.io.File;

public class DeleteDirectory {
    public static void main(String[] args) {
        File directory = new File(args[0]);
        System.out.println(directory.delete());
        System.out.println(directory.exists());
    }
}
