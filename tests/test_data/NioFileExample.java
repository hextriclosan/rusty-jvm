package samples.nio.niofileexample;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardOpenOption;

public class NioFileExample {
    public static void main(String[] args) throws IOException {
        if (args.length < 2) {
            System.err.println("Usage: java NioFileExample <operation> <file_name> [<content>]");
            return;
        }

        String operation = args[0];
        String name = args[1];
        if ("--create-dirs".equals(operation)) {
            createDirectories(name);
        } else if ("--write-file".equals(operation)) {
            String content = args[2];
            writeFile(name, content);
        } else if ("--read-file".equals(operation)) {
            String content = readFile(name);
            System.out.println("File content: " + content);
        } else if ("--delete".equals(operation)) {
            delete(name);
        } else if ("--is-writable".equals(operation)) {
            isWritable(name);
        } else {
            System.err.println("Unknown operation: " + operation);
        }
    }

    private static void delete(String name) throws IOException {
        Path path = Paths.get(name);
        Files.deleteIfExists(path);
        System.out.println("Deleted: " + path);
    }

    private static String readFile(String name) throws IOException {
        Path path = Paths.get(name);
        return Files.readString(path);
    }

    private static void writeFile(String name, String content) throws IOException {
        Path path = Paths.get(name);
        Files.writeString(path, content, StandardOpenOption.CREATE, StandardOpenOption.TRUNCATE_EXISTING);
        System.out.println("Written: " + path);
    }

    private static void createDirectories(String name) throws IOException {
        Path path = Paths.get(name);
        Files.createDirectories(path);
        System.out.println("Created directories: " + path);
    }

    private static void isWritable(String name) {
        Path path = Paths.get(name);
        boolean writable = Files.isWritable(path);
        System.out.println(path + " is writable: " + writable);
    }
}
