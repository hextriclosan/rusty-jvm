package samples.io.randomaccessfilepointer;

import java.io.RandomAccessFile;

public class RandomAccessFilePointer {
    public static void main(String[] args) throws Exception {
        try (RandomAccessFile file = new RandomAccessFile(args[0], "rw")) {
            file.write("data".getBytes());
            System.out.println(file.getFilePointer());
            file.seek(1);
            System.out.println(file.getFilePointer());
            file.read(new byte[1]);
            System.out.println(file.getFilePointer());
        }
    }
}
