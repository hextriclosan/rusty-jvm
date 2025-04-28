package com.github.hextriclosan.rustyjvm;

import org.objectweb.asm.ClassWriter;
import org.objectweb.asm.MethodVisitor;
import org.objectweb.asm.Opcodes;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;

public class OperandStackOverflowGenerator {
    private static final String PACKAGE = "samples/invalidprograms/operandstackoverflow";
    private static final String CLASS_NAME = "OperandStackOverflowExample";
    private static final String FULL_NAME = PACKAGE + "/" + CLASS_NAME;

    public static void main(String[] args) throws IOException {
        ClassWriter cw = new ClassWriter(0);
        cw.visit(Opcodes.V23, Opcodes.ACC_PUBLIC, FULL_NAME, null, "java/lang/Object", null);

        MethodVisitor mv = cw.visitMethod(Opcodes.ACC_PRIVATE | Opcodes.ACC_STATIC, "returnInt", "()I", null, null);
        mv.visitCode();

        // Push three ints onto the stack
        mv.visitLdcInsn(10);
        mv.visitLdcInsn(20);
        mv.visitLdcInsn(30);

        // return 30
        mv.visitInsn(Opcodes.IRETURN);
        mv.visitMaxs(2, 0);
        mv.visitEnd();

        // Generate `public static void main(String[])`
        mv = cw.visitMethod(Opcodes.ACC_PUBLIC | Opcodes.ACC_STATIC, "main", "([Ljava/lang/String;)V", null, null);
        mv.visitCode();

        mv.visitMethodInsn(Opcodes.INVOKESTATIC, FULL_NAME, "returnInt", "()I", false);

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
