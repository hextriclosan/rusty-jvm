package com.github.hextriclosan.rustyjvm;

import org.objectweb.asm.ClassWriter;
import org.objectweb.asm.MethodVisitor;
import org.objectweb.asm.Opcodes;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;

import static com.github.hextriclosan.rustyjvm.Constants.OUTPUT_PATH;

public class SwapGenerator {
    private static final String PACKAGE = "samples/opcodes/swap";
    private static final String CLASS_NAME = "SwapGeneratedExample";
    private static final String FULL_NAME = PACKAGE + "/" + CLASS_NAME;

    public static void main(String[] args) throws IOException {
        ClassWriter cw = new ClassWriter(ClassWriter.COMPUTE_FRAMES | ClassWriter.COMPUTE_MAXS);
        cw.visit(Opcodes.V23, Opcodes.ACC_PUBLIC, FULL_NAME, null, "java/lang/Object", null);

        // Generate `int nopExample()`
        MethodVisitor mv = cw.visitMethod(Opcodes.ACC_PRIVATE | Opcodes.ACC_STATIC, "exampleMethod", "()I", null, null);
        mv.visitCode();

        // Push two ints onto the stack
        mv.visitLdcInsn(5);   // int 5
        mv.visitLdcInsn(7);   // int 7

        mv.visitInsn(Opcodes.SWAP);

        // Add the two ints
        mv.visitInsn(Opcodes.ISUB);

        // return result
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
