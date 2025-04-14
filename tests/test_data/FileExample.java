// javac -XDstringConcat=inline  -d . FileExample.java
package samples.io.fileexample;

import java.io.File;
import java.io.FileWriter;
import java.io.IOException;

public class FileExample {
    private static final String DIR_NAME = "../tmp";
    private static final String FILE_NAME = "file_example_test.txt";
    private static final String PATH = DIR_NAME + "/" + FILE_NAME;

    public static void main(String[] args) throws IOException {
        File file = new File(PATH);

        printFileInfo(file);

        createFile(file);
        createFile(file);

//         try (FileWriter fileWriter = new FileWriter(file, true)) {
//             fileWriter.write("Hello, World!\n");
//         }

        printFileInfo(file);


        File dir = new File(DIR_NAME);
        printFileInfo(dir);
//         String[] files = dir.list(); Native method java/io/WinNTFileSystem:createFileExclusively0:(Ljava/lang/String;)Z not found
//         if (files != null) {
//             for (String name : files) {
//                 System.out.println(" - " + name);
//             }
//         }

        deleteFile(file);
        deleteFile(file);
    }

    private static void printFileInfo(File file) throws IOException {
        System.out.println("File info: " + file);
        System.out.println("\tAbsolute path: " + file.getAbsolutePath());
        System.out.println("\tCanonical path: " + file.getCanonicalPath());
        System.out.println("\tPath: " + file.getPath());
        System.out.println("\tName: " + file.getName());
        System.out.println("\tParent: " + file.getParent());
        System.out.println("\tParent file: " + file.getParentFile());
        System.out.println("\tIs absolute: " + file.isAbsolute());
        System.out.println("\tFile exists: " + file.exists());
        System.out.println("\tIs file: " + file.isFile());
        System.out.println("\tIs directory: " + file.isDirectory());
        System.out.println("\tIs hidden: " + file.isHidden());
//         System.out.println("\tIs writable: " + file.canWrite()); Native method java/io/WinNTFileSystem:checkAccess0:(Ljava/io/File;I)Z not found
//         System.out.println("\tIs readable: " + file.canRead()); Native method java/io/WinNTFileSystem:checkAccess0:(Ljava/io/File;I)Z not found
//         System.out.println("\tIs executable: " + file.canExecute()); Native method java/io/WinNTFileSystem:checkAccess0:(Ljava/io/File;I)Z not found
//         System.out.println("\tFile length: " + file.length() + " bytes"); Native method java/io/WinNTFileSystem:getLength0:(Ljava/io/File;)J not found
//         System.out.println("\tLast modified: " + file.lastModified()); Native method java/io/WinNTFileSystem:getLastModifiedTime0:(Ljava/io/File;)J not found
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
