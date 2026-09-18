package samples.io.filedescriptorsync;

import java.io.FileOutputStream;
import java.io.FileDescriptor;
import java.io.SyncFailedException;

public class FileDescriptorSync {
    public static void main(String[] args) throws Exception {
        FileDescriptor descriptor;
        try (FileOutputStream output = new FileOutputStream(args[0])) {
            output.write("synced".getBytes());
            descriptor = output.getFD();
            descriptor.sync();
        }
        System.out.println("completed");
        try {
            descriptor.sync();
        } catch (SyncFailedException error) {
            System.out.println(error.getClass().getSimpleName());
        }
    }
}
