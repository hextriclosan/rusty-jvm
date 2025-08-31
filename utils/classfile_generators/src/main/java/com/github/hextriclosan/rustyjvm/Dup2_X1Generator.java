package com.github.hextriclosan.rustyjvm;

import org.objectweb.asm.ClassWriter;
import org.objectweb.asm.MethodVisitor;
import org.objectweb.asm.Opcodes;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;

import static com.github.hextriclosan.rustyjvm.Constants.OUTPUT_PATH;

public class Dup2_X1Generator {
    private static final String PACKAGE = "samples/opcodes/dup2_x1";
    private static final String CLASS_NAME = "Dup2_X1GeneratedExample";
    private static final String FULL_NAME = PACKAGE + "/" + CLASS_NAME;

    public static void main(String[] args) throws IOException {
        ClassWriter cw = new ClassWriter(ClassWriter.COMPUTE_FRAMES | ClassWriter.COMPUTE_MAXS);
        cw.visit(Opcodes.V23, Opcodes.ACC_PUBLIC, FULL_NAME, null, "java/lang/Object", null);

        MethodVisitor mv = cw.visitMethod(Opcodes.ACC_PRIVATE | Opcodes.ACC_STATIC, "exampleMethod", "()I", null, null);
        mv.visitCode();

        // Stack: [3]
        mv.visitInsn(Opcodes.ICONST_3);
        // Stack: [3, 2]
        mv.visitInsn(Opcodes.ICONST_2);
        // Stack: [3, 2, 1]
        mv.visitInsn(Opcodes.ICONST_1);

        // Apply DUP2_X1
        // Stack: [2, 1, 3, 2, 1]
        mv.visitInsn(Opcodes.DUP2_X1);

        // Now let's pop the top 5 values into locals
        // Store: a=2, b=1, c=3, d=2, e=1
        mv.visitVarInsn(Opcodes.ISTORE, 0); // e
        mv.visitVarInsn(Opcodes.ISTORE, 1); // d
        mv.visitVarInsn(Opcodes.ISTORE, 2); // c
        mv.visitVarInsn(Opcodes.ISTORE, 3); // b
        mv.visitVarInsn(Opcodes.ISTORE, 4); // a

        // Compute: a*10000 + b*1000 + c*100 + d*10 + e
        mv.visitVarInsn(Opcodes.ILOAD, 4); // a
        mv.visitIntInsn(Opcodes.SIPUSH, 10000);
        mv.visitInsn(Opcodes.IMUL);

        mv.visitVarInsn(Opcodes.ILOAD, 3); // b
        mv.visitIntInsn(Opcodes.SIPUSH, 1000);
        mv.visitInsn(Opcodes.IMUL);
        mv.visitInsn(Opcodes.IADD);

        mv.visitVarInsn(Opcodes.ILOAD, 2); // c
        mv.visitIntInsn(Opcodes.BIPUSH, 100);
        mv.visitInsn(Opcodes.IMUL);
        mv.visitInsn(Opcodes.IADD);

        mv.visitVarInsn(Opcodes.ILOAD, 1); // d
        mv.visitIntInsn(Opcodes.BIPUSH, 10);
        mv.visitInsn(Opcodes.IMUL);
        mv.visitInsn(Opcodes.IADD);

        mv.visitVarInsn(Opcodes.ILOAD, 0); // e
        mv.visitInsn(Opcodes.IADD);

        // Return result
        mv.visitInsn(Opcodes.IRETURN);

        mv.visitMaxs(0, 0); // Auto-computed
        mv.visitEnd();

        // Generate `public static void main(String[])`
        mv = cw.visitMethod(Opcodes.ACC_PUBLIC | Opcodes.ACC_STATIC, "main", "([Ljava/lang/String;)V", null, null);
        mv.visitCode();

        mv.visitFieldInsn(Opcodes.GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");

        mv.visitMethodInsn(Opcodes.INVOKESTATIC, FULL_NAME, "exampleMethod", "()I", false);

        mv.visitMethodInsn(Opcodes.INVOKEVIRTUAL, "java/io/PrintStream", "println", "(I)V", false);

        mv.visitInsn(Opcodes.RETURN);
        mv.visitMaxs(0, 0); // Auto-computed
        mv.visitEnd();

        cw.visitEnd();

        File outputDir = new File(OUTPUT_PATH + '/' + PACKAGE);
        outputDir.mkdirs();
        try (FileOutputStream fos = new FileOutputStream(new File(outputDir, CLASS_NAME + ".class"))) {
            fos.write(cw.toByteArray());
        }
        System.out.println("class written successfully.");
    }
}
