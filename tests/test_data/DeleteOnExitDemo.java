package samples.io.file.deleteonexitdemo;

import java.io.File;
import java.io.FileInputStream;

public class DeleteOnExitDemo {
    public static void main(String[] args) throws Exception {
        File file = new File(args[0]);
        file.deleteOnExit();

        try (FileInputStream in = new FileInputStream(file)) {
            System.out.write(in.readAllBytes());
        }
    }
}
