package samples.io.fileinputstreamthrowexample;

import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStream;

public class FileInputStreamThrowExample {
    public static void main(String[] args) throws IOException {
        String filePath = args[0];
        InputStream fos = new FileInputStream(filePath);
        char c = (char) fos.read(); // works
        fos.close();
        System.out.println(c);

        File file = new File(filePath);
        String parentDir = file.getAbsoluteFile().getParent();
        try {
            openFile(parentDir); // <-- guaranteed IOException
        } catch (IOException e) {
            System.out.println("openFile: " + e);
        }

        try {
            readByte(fos); // <-- guaranteed IOException
        } catch (IOException e) {
            System.out.println("readByte: " + e);
        }

        try {
            readBytes(fos, new byte[2]); // <-- guaranteed IOException
        } catch (IOException e) {
            System.out.println("readBytes: " + e);
        }

        try {
            readAllBytes(fos); // <-- guaranteed IOException
        } catch (IOException e) {
            System.out.println("readAllBytes: " + e);
        }
    }

    private static void openFile(String path) throws IOException {
        try (var fis = new java.io.FileInputStream(path)) {
        }
    }

    private static int readByte(InputStream is) throws IOException {
        return is.read();
    }

    private static void readBytes(InputStream is, byte[] bytes) throws IOException {
        is.read(bytes);
    }

    private static byte[] readAllBytes(InputStream is) throws IOException {
        return is.readAllBytes();
    }
}
