package samples.io.fileoperations;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.util.Arrays;

/**
 * Creates, changes and deletes files in the directory given as the argument, through java.io.File. Unix and
 * Windows results differ in places, which a comparison with the reference JVM covers.
 */
public class FileOperations {
    public static void main(String[] args) throws IOException {
        File base = new File(args[0]);
        File missing = new File(base, "missing");

        // createDirectory0
        File dir = new File(base, "dir");
        print("mkdir", dir.mkdir());
        print("mkdir existing", dir.mkdir());
        print("mkdir under a missing parent", new File(missing, "child").mkdir());
        print("mkdirs", new File(base, "a/b/c").mkdirs());
        print("isDirectory", dir.isDirectory());
        File file = new File(dir, "file.txt");
        try (FileOutputStream out = new FileOutputStream(file)) {
            out.write(new byte[] {1, 2, 3});
        }
        print("mkdir over a file", file.mkdir());

        // getLastModifiedTime0 and setLastModifiedTime0
        print("setLastModified", file.setLastModified(1_600_000_000_123L));
        print("lastModified", file.lastModified());
        print("setLastModified of a directory", dir.setLastModified(1_500_000_000_000L));
        print("lastModified of a directory", dir.lastModified());
        print("setLastModified of a missing file", missing.setLastModified(1_000L));
        print("lastModified of a missing file", missing.lastModified());
        try {
            file.setLastModified(-1);
        } catch (IllegalArgumentException e) {
            print("setLastModified to a negative time", e.getMessage());
        }

        // setReadOnly0 and setPermission0
        print("setReadOnly", file.setReadOnly());
        print("canWrite", file.canWrite());
        print("setLastModified of a read-only file", file.setLastModified(1_600_000_000_456L));
        print("lastModified of a read-only file", file.lastModified());
        print("setWritable", file.setWritable(true));
        print("canWrite", file.canWrite());
        print("setWritable(false, false)", file.setWritable(false, false));
        print("canWrite", file.canWrite());
        print("setWritable(true, true)", file.setWritable(true, true));
        print("canWrite", file.canWrite());
        print("setExecutable", file.setExecutable(true));
        print("canExecute", file.canExecute());
        print("setExecutable(false)", file.setExecutable(false));
        print("setReadable(false, false)", file.setReadable(false, false));
        print("setReadable", file.setReadable(true));
        print("canRead", file.canRead());
        print("setWritable of a missing file", missing.setWritable(true));
        print("setReadable of a missing file", missing.setReadable(true));
        print("setReadOnly of a missing file", missing.setReadOnly());
        File locked = new File(base, "locked");
        locked.mkdir();
        print("setReadOnly of a directory", locked.setReadOnly());
        print("setWritable(false) of a directory", locked.setWritable(false));
        print("setWritable of a directory", locked.setWritable(true));
        print("delete a directory", locked.delete());

        // rename0
        File renamed = new File(dir, "renamed.txt");
        print("renameTo", file.renameTo(renamed));
        print("exists after renameTo", file.exists());
        print("length after renameTo", renamed.length());
        print("lastModified after renameTo", renamed.lastModified());
        File other = new File(dir, "other.txt");
        other.createNewFile();
        print("renameTo an existing file", other.renameTo(renamed));
        print("length of the existing file", renamed.length());
        print("renameTo from a missing file", missing.renameTo(new File(base, "never")));
        File moved = new File(base, "moved");
        print("renameTo of a directory", dir.renameTo(moved));
        print("files of a moved directory", Arrays.toString(sorted(moved.list())));

        // delete0
        print("delete a directory with files", moved.delete());
        for (String name : sorted(moved.list())) {
            print("delete " + name, new File(moved, name).delete());
        }
        print("delete an empty directory", moved.delete());
        print("delete a missing file", moved.delete());
        // links that the test makes on Unix only
        print("delete a dangling link", new File(base, "dangling").delete());
        print("delete a link to a directory", new File(base, "linked").delete());
        print("isDirectory of the link target", new File(base, "a").isDirectory());

        // getSpace0
        long total = base.getTotalSpace();
        long free = base.getFreeSpace();
        long usable = base.getUsableSpace();
        print("getTotalSpace", total);
        print("getFreeSpace within getTotalSpace", 0 < free && free <= total);
        print("getUsableSpace within getFreeSpace", 0 < usable && usable <= free);
        print("getTotalSpace of a missing file", missing.getTotalSpace());

        print("files", Arrays.toString(sorted(base.list())));
    }

    private static String[] sorted(String[] names) {
        Arrays.sort(names);
        return names;
    }

    private static void print(String label, Object value) {
        System.out.println(label + ": " + value);
    }
}
