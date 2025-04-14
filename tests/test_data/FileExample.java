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

//         createFile(file); Native method java/io/WinNTFileSystem:createFileExclusively0:(Ljava/lang/String;)Z not found
//         createFile(file); Native method java/io/WinNTFileSystem:createFileExclusively0:(Ljava/lang/String;)Z not found

//         try (FileWriter fileWriter = new FileWriter(file, true)) {
//             fileWriter.write("Hello, World!\n");
//         }

//         printFileInfo(file); uncomment after creating the file


        File dir = new File(DIR_NAME);
        printFileInfo(dir);
//         String[] files = dir.list(); Native method java/io/WinNTFileSystem:createFileExclusively0:(Ljava/lang/String;)Z not found
//         if (files != null) {
//             for (String name : files) {
//                 System.out.println(" - " + name);
//             }
//         }

//         deleteFile(file); Native method java/io/WinNTFileSystem:delete0:(Ljava/io/File;)Z not found
//         deleteFile(file); Native method java/io/WinNTFileSystem:delete0:(Ljava/io/File;)Z not found
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
//         System.out.println("File exists: " + file.exists()); Native method java/io/WinNTFileSystem:getBooleanAttributes0:(Ljava/io/File;)I not found
//         System.out.println("Is file: " + file.isFile()); Native method java/io/WinNTFileSystem:getBooleanAttributes0:(Ljava/io/File;)I not found
//         System.out.println("Is directory: " + file.isDirectory()); Native method java/io/WinNTFileSystem:getBooleanAttributes0:(Ljava/io/File;)I not found
//         System.out.println("Is hidden: " + file.isHidden()); Native method java/io/WinNTFileSystem:getBooleanAttributes0:(Ljava/io/File;)I not found
//         System.out.println("Is writable: " + file.canWrite()); Native method java/io/WinNTFileSystem:checkAccess0:(Ljava/io/File;I)Z not found
//         System.out.println("Is readable: " + file.canRead()); Native method java/io/WinNTFileSystem:checkAccess0:(Ljava/io/File;I)Z not found
//         System.out.println("Is executable: " + file.canExecute()); Native method java/io/WinNTFileSystem:checkAccess0:(Ljava/io/File;I)Z not found
//         System.out.println("File length: " + file.length() + " bytes"); Native method java/io/WinNTFileSystem:getLength0:(Ljava/io/File;)J not found
//         System.out.println("Last modified: " + file.lastModified()); Native method java/io/WinNTFileSystem:getLastModifiedTime0:(Ljava/io/File;)J not found
    }

    private static void createFile(File file) throws IOException {
        if (file.createNewFile()) {
            System.out.println("File created: " + file.getName());
        } else {
            System.out.println("File already exists.");
        }
    }

    private static void deleteFile(File file) {
        if (file.delete()) {
            System.out.println("Deleted the file: " + file.getName());
        } else {
            System.out.println("Failed to delete the file.");
        }
    }
}
