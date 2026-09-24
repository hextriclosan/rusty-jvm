package samples.io.listdirectory;

import java.io.File;
import java.util.Arrays;

public class ListDirectory {
    public static void main(String[] args) {
        String[] entries = new File(args[0]).list();
        Arrays.sort(entries);
        System.out.println(Arrays.toString(entries));
    }
}
