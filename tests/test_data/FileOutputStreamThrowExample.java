package samples.io.fileoutputstreamthrowexample;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.OutputStream;

public class FileOutputStreamThrowExample {
    public static void main(String[] args) throws IOException {
        String filePath = args[0];

        File file = new File(filePath);
        String parentDir = file.getAbsoluteFile().getParent();
        try {
            openFile(parentDir); // <-- guaranteed IOException
        } catch (IOException e) {
            System.out.println("openFile: " + e);
        }

        FileOutputStream fos = new FileOutputStream(filePath);
        fos.write('A'); // works
        fos.close();

        try {
            writeByte(fos, 'B'); // <-- guaranteed IOException
        } catch (IOException e) {
            System.out.println("writeByte: " + e);
        }

        try {
            writeBytes(fos, "CDE".getBytes()); // <-- guaranteed IOException
        } catch (IOException e) {
            System.out.println("writeBytes: " + e);
        }
    }

    private static void openFile(String path) throws IOException {
        try (OutputStream fos = new FileOutputStream(path)) {
            fos.write(1);
        }
    }

    private static void writeByte(OutputStream fos, int value) throws IOException {
        fos.write(value);
    }

    private static void writeBytes(OutputStream fos, byte[] bytes) throws IOException {
        fos.write(bytes);
    }
}
