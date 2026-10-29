package samples.io.randomaccessfilesinglebyte;

import java.io.RandomAccessFile;

public class RandomAccessFileSingleByte {
    public static void main(String[] args) throws Exception {
        try (RandomAccessFile file = new RandomAccessFile(args[0], "rw")) {
            file.write('A');
            file.seek(0);
            System.out.println(file.read());
            System.out.println(file.read());
        }
    }
}
