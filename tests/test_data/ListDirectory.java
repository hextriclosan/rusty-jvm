package samples.io.listdirectory;

import java.io.File;
import java.util.Arrays;

public class ListDirectory {
    public static void main(String[] args) {
        for (String path : args) {
            String[] entries = new File(path).list();
            if (entries == null) {
                System.out.println("null");
                continue;
            }
            Arrays.sort(entries);
            String[] escaped = Arrays.stream(entries).map(ListDirectory::escape).toArray(String[]::new);
            System.out.println(Arrays.toString(escaped));
        }
    }

    // Keep the output independent of the stdout encoding
    private static String escape(String name) {
        StringBuilder sb = new StringBuilder();
        for (char c : name.toCharArray()) {
            if (c < 0x80) {
                sb.append(c);
            } else {
                sb.append(String.format("\\u%04X", (int) c));
            }
        }
        return sb.toString();
    }
}
