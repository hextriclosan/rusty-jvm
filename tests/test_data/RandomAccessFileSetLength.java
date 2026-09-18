package samples.io.randomaccessfilesetlength;

import java.io.RandomAccessFile;

public class RandomAccessFileSetLength {
    public static void main(String[] args) throws Exception {
        try (RandomAccessFile file = new RandomAccessFile(args[0], "rw")) {
            file.write("abcdef".getBytes());
            file.setLength(3);
            file.write(new byte[]{'X'});
            System.out.println(file.length());
            file.setLength(6);
            System.out.println(file.length());
        }
    }
}
