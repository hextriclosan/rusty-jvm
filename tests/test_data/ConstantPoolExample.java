// javac --add-exports java.base/jdk.internal.reflect=ALL-UNNAMED --patch-module java.base=. -d . ConstantPoolExample.java
// java --patch-module java.base=. java.lang.ConstantPoolExample

// This test is put to java.lang package for calling package-private method
// It's not so nice but bad test is better than no test
package java.lang;

import jdk.internal.reflect.ConstantPool;

public class ConstantPoolExample {
    public static void main(String[] args) {
        ConstantPool cp = SuppressWarnings.class.getConstantPool();

        int size = cp.getSize();
        System.out.print("Constant pool size: ");
        System.out.println(size);

        // Iterate through constant pool entries (indices start from 1)
        for (int i = 1; i < size; i++) {
            // Print details about each entry
            ConstantPool.Tag tag = cp.getTagAt(i);
            System.out.print(i);
            System.out.print(": ");
            System.out.print(tag);
            System.out.print(" (");
            System.out.print(tag == ConstantPool.Tag.UTF8 ? cp.getUTF8At(i): "");
            System.out.println(")");
        }
    }
}
