package samples.io.filedescriptorsync;

import java.io.FileDescriptor;
import java.io.FileInputStream;
import java.io.FileOutputStream;
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

        // A read-only descriptor has nothing to flush
        try (FileInputStream input = new FileInputStream(args[0])) {
            input.getFD().sync();
            System.out.println("read-only completed");
        }

        sync("closed", descriptor);
        sync("invalid", new FileDescriptor());
    }

    private static void sync(String label, FileDescriptor descriptor) {
        try {
            descriptor.sync();
            System.out.println(label + ": completed");
        } catch (SyncFailedException error) {
            System.out.println(label + ": " + error);
        }
    }
}
