package samples.io.fileexample;

import java.io.File;
import java.io.FileWriter;
import java.io.IOException;

public class FileExample {
    public static void main(String[] args) throws IOException {
        if (args.length < 2) {
            System.err.println("Usage: java FileExample <operation> <file_name>");
            return;
        }

        String operation = args[0];
        String fileName = args[1];
        File file = new File(fileName);
        if ("--delete".equals(operation)) {
            deleteFile(file);
        } else if ("--create".equals(operation)) {
            createFile(file);
            try (FileWriter fileWriter = new FileWriter(file, true)) {
                fileWriter.write("Hello, World!\n");
            }
        } else if ("--list".equals(operation)) {
         String[] files = file.list(); // Native method java/io/WinNTFileSystem:createFileExclusively0:(Ljava/lang/String;)Z not found
         if (files != null) {
             for (String name : files) {
                 System.out.println(" - " + name);
             }
         }
        } else if ("--info".equals(operation)) {
            printFileInfo(file);
        } else {
            System.err.println("Unknown operation: " + operation);
        }
    }

    private static void printFileInfo(File file) throws IOException {
        System.out.println("File info: " + file);
        System.out.println("Absolute path: " + file.getAbsolutePath());
        System.out.println("Canonical path: " + file.getCanonicalPath());
        System.out.println("Path: " + file.getPath());
        System.out.println("Name: " + file.getName());
        System.out.println("Parent: " + file.getParent());
        System.out.println("Parent file: " + file.getParentFile());
        System.out.println("Is absolute: " + file.isAbsolute());
        System.out.println("File exists: " + file.exists());
        System.out.println("Is file: " + file.isFile());
        System.out.println("Is directory: " + file.isDirectory());
        System.out.println("Is hidden: " + file.isHidden());
        System.out.println("Is writable: " + file.canWrite());
        System.out.println("Is readable: " + file.canRead());
        System.out.println("Is executable: " + file.canExecute());
//         System.out.println("File length: " + file.length() + " bytes"); Native method java/io/WinNTFileSystem:getLength0:(Ljava/io/File;)J not found
//         System.out.println("Last modified: " + file.lastModified()); Native method java/io/WinNTFileSystem:getLastModifiedTime0:(Ljava/io/File;)J not found
    }

    private static void createFile(File file) throws IOException {
        if (file.createNewFile()) {
            System.out.println(file.getName() + ": created");
        } else {
            System.out.println(file.getName() + ": already exists");
        }
    }

    private static void deleteFile(File file) {
        if (file.delete()) {
            System.out.println(file.getName() + ": deleted");
        } else {
            System.out.println(file.getName() + ": does not exist");
        }
    }
}
