package samples.io.filespace;

import java.io.File;

public class FileSpace {
    public static void main(String[] args) {
        File file = new File(args[0]);
        long total = file.getTotalSpace();
        long free = file.getFreeSpace();
        long usable = file.getUsableSpace();
        System.out.println(total > 0);
        System.out.println(free >= 0 && free <= total);
        System.out.println(usable >= 0 && usable <= free);
    }
}
