package com.github.hextriclosan.rustyjvm;

import org.objectweb.asm.ClassWriter;
import org.objectweb.asm.MethodVisitor;
import org.objectweb.asm.Opcodes;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;

import static com.github.hextriclosan.rustyjvm.Constants.OUTPUT_PATH;

public class Dup2_X2Generator {
    private static final String PACKAGE = "samples/opcodes/dup2_x2";
    private static final String CLASS_NAME = "Dup2_X2GeneratedExample";
    private static final String FULL_NAME = PACKAGE + "/" + CLASS_NAME;

    public static void main(String[] args) throws IOException {
        ClassWriter cw = new ClassWriter(ClassWriter.COMPUTE_FRAMES | ClassWriter.COMPUTE_MAXS);
        cw.visit(Opcodes.V23, Opcodes.ACC_PUBLIC, FULL_NAME, null, "java/lang/Object", null);

        MethodVisitor mv = cw.visitMethod(Opcodes.ACC_PRIVATE | Opcodes.ACC_STATIC, "exampleMethod", "()I", null, null);
        mv.visitCode();

        // Push values: 4, 3, 2, 1
        mv.visitInsn(Opcodes.ICONST_4); // value4
        mv.visitInsn(Opcodes.ICONST_3); // value3
        mv.visitInsn(Opcodes.ICONST_2); // value2
        mv.visitInsn(Opcodes.ICONST_1); // value1

        // Stack: 4 3 2 1
        mv.visitInsn(Opcodes.DUP2_X2); // After: 2 1 4 3 2 1

        // Store in reverse order
        mv.visitVarInsn(Opcodes.ISTORE, 0); // x = 1
        mv.visitVarInsn(Opcodes.ISTORE, 1); // e = 2
        mv.visitVarInsn(Opcodes.ISTORE, 2); // d = 3
        mv.visitVarInsn(Opcodes.ISTORE, 3); // c = 4
        mv.visitVarInsn(Opcodes.ISTORE, 4); // b = 1
        mv.visitVarInsn(Opcodes.ISTORE, 5); // a = 2

        // Return a*100000 + b*10000 + c*1000 + d*100 + e*10 + x
        mv.visitVarInsn(Opcodes.ILOAD, 5); // a
        mv.visitLdcInsn(100000);
        mv.visitInsn(Opcodes.IMUL);

        mv.visitVarInsn(Opcodes.ILOAD, 4); // b
        mv.visitIntInsn(Opcodes.SIPUSH, 10000);
        mv.visitInsn(Opcodes.IMUL);
        mv.visitInsn(Opcodes.IADD);

        mv.visitVarInsn(Opcodes.ILOAD, 3); // c
        mv.visitIntInsn(Opcodes.SIPUSH, 1000);
        mv.visitInsn(Opcodes.IMUL);
        mv.visitInsn(Opcodes.IADD);

        mv.visitVarInsn(Opcodes.ILOAD, 2); // d
        mv.visitIntInsn(Opcodes.BIPUSH, 100);
        mv.visitInsn(Opcodes.IMUL);
        mv.visitInsn(Opcodes.IADD);

        mv.visitVarInsn(Opcodes.ILOAD, 1); // e
        mv.visitIntInsn(Opcodes.BIPUSH, 10);
        mv.visitInsn(Opcodes.IMUL);
        mv.visitInsn(Opcodes.IADD);

        mv.visitVarInsn(Opcodes.ILOAD, 0); // x
        mv.visitInsn(Opcodes.IADD);

        mv.visitInsn(Opcodes.IRETURN);
        mv.visitMaxs(0, 0);
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
