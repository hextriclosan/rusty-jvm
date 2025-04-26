package com.github.hextriclosan.rustyjvm;

import org.objectweb.asm.ClassWriter;
import org.objectweb.asm.MethodVisitor;
import org.objectweb.asm.Opcodes;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;

public class Pop2Generator {
    private static final String PACKAGE = "samples/opcodes/pop2";
    private static final String CLASS_NAME = "Pop2GeneratedExample";
    private static final String FULL_NAME = PACKAGE + "/" + CLASS_NAME;

    public static void main(String[] args) throws IOException {
        ClassWriter cw = new ClassWriter(0);
        cw.visit(Opcodes.V23, Opcodes.ACC_PUBLIC, FULL_NAME, null, "java/lang/Object", null);

        // Generate `int pop2Example()`
        MethodVisitor mv = cw.visitMethod(Opcodes.ACC_PRIVATE | Opcodes.ACC_STATIC, "pop2Example", "()I", null, null);
        mv.visitCode();

        // Push two ints onto the stack
        mv.visitLdcInsn(5);   // int 5
        mv.visitLdcInsn(7);   // int 7

        // Remove them (so they don't interfere)
        mv.visitInsn(Opcodes.POP2);

        // Now push 1000
        mv.visitLdcInsn(1000);

        // return 1000
        mv.visitInsn(Opcodes.IRETURN);
        mv.visitMaxs(2, 0);
        mv.visitEnd();

        // Generate `public static void main(String[])`
        mv = cw.visitMethod(Opcodes.ACC_PUBLIC | Opcodes.ACC_STATIC, "main", "([Ljava/lang/String;)V", null, null);
        mv.visitCode();

        mv.visitFieldInsn(Opcodes.GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");

        mv.visitMethodInsn(Opcodes.INVOKESTATIC, FULL_NAME, "pop2Example", "()I", false);

        mv.visitMethodInsn(Opcodes.INVOKEVIRTUAL, "java/io/PrintStream", "println", "(I)V", false);

        mv.visitInsn(Opcodes.RETURN);
        mv.visitMaxs(2, 1);
        mv.visitEnd();

        cw.visitEnd();

        File outputDir = new File("../../tests/test_data/" + PACKAGE);
        outputDir.mkdirs();
        try (FileOutputStream fos = new FileOutputStream(new File(outputDir, CLASS_NAME + ".class"))) {
            fos.write(cw.toByteArray());
        }
        System.out.println("class written successfully.");
    }
}
