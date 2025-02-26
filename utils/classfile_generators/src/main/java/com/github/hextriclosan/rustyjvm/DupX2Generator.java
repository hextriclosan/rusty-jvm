package com.github.hextriclosan.rustyjvm;

import org.objectweb.asm.MethodVisitor;
import org.objectweb.asm.Opcodes;
import org.objectweb.asm.ClassWriter;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;

public class DupX2Generator {
    private static final String PACKAGE = "samples/opcodes/dup_x2";
    private static final String CLASS_NAME = "DupX2GeneratedExample";
    private static final String FULL_NAME = PACKAGE + "/" + CLASS_NAME;

    public static void main(String[] args) throws IOException {
        ClassWriter cw = new ClassWriter(0);
        cw.visit(Opcodes.V23, Opcodes.ACC_PUBLIC, FULL_NAME, null, "java/lang/Object", null);

        // Generate `float dupX2Example(float, float, float)`
        MethodVisitor mv = cw.visitMethod(Opcodes.ACC_PRIVATE | Opcodes.ACC_STATIC, "dupX2Example", "(FFF)F", null, null);
        mv.visitCode();

        // Stack: [ a, b, c ]
        mv.visitVarInsn(Opcodes.FLOAD, 0); // Load a
        mv.visitVarInsn(Opcodes.FLOAD, 1); // Load b
        mv.visitVarInsn(Opcodes.FLOAD, 2); // Load c

        mv.visitInsn(Opcodes.DUP_X2);  // Stack: [ c, a, b, c ]
        mv.visitInsn(Opcodes.FADD);    // Stack: [ c, a, (b + c) ]
        mv.visitInsn(Opcodes.FMUL);    // Stack: [ c, (a * (b + c)) ]
        mv.visitInsn(Opcodes.FADD);    // Stack: [ (c + (a * (b + c))) ]

        mv.visitInsn(Opcodes.FRETURN); // Return result

        mv.visitMaxs(4, 3);
        mv.visitEnd();

        // Generate `public static void main(String[])`
        mv = cw.visitMethod(Opcodes.ACC_PUBLIC | Opcodes.ACC_STATIC, "main", "([Ljava/lang/String;)V", null, null);
        mv.visitCode();

        mv.visitFieldInsn(Opcodes.GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");

        // Call `dupX2Example(2, 30, 400)` -> Stack: [20, 30, 400]
        mv.visitLdcInsn(2f);
        mv.visitLdcInsn(30f);
        mv.visitLdcInsn(400f);

        // Call float dupX2Example(float, float, float)
        mv.visitMethodInsn(Opcodes.INVOKESTATIC, FULL_NAME, "dupX2Example", "(FFF)F", false);

        // PrintStream.println(float) call
        mv.visitMethodInsn(Opcodes.INVOKEVIRTUAL, "java/io/PrintStream", "println", "(F)V", false);

        mv.visitInsn(Opcodes.RETURN);
        mv.visitMaxs(4, 1);
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
