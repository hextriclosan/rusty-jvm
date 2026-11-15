package com.github.hextriclosan.rustyjvm;

import org.objectweb.asm.ClassWriter;
import org.objectweb.asm.Label;
import org.objectweb.asm.MethodVisitor;
import org.objectweb.asm.Opcodes;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;

import static com.github.hextriclosan.rustyjvm.Constants.OUTPUT_PATH;

/**
 * A {@code monitorexit} on an object the method never entered: javac cannot produce it, and JVMS
 * §6.5 requires {@code IllegalMonitorStateException}, thrown at the instruction so that a handler
 * in the caller catches it.
 */
public class UnownedMonitorExitGenerator {
    private static final String PACKAGE = "samples/invalidprograms/unownedmonitorexit";
    private static final String CLASS_NAME = "UnownedMonitorExitExample";
    private static final String FULL_NAME = PACKAGE + "/" + CLASS_NAME;

    public static void main(String[] args) throws IOException {
        ClassWriter cw = new ClassWriter(ClassWriter.COMPUTE_FRAMES);
        cw.visit(Opcodes.V23, Opcodes.ACC_PUBLIC, FULL_NAME, null, "java/lang/Object", null);

        // Generate `private static void exitWithoutEnter(Object lock)`: `aload_0; monitorexit; return`
        MethodVisitor mv = cw.visitMethod(Opcodes.ACC_PRIVATE | Opcodes.ACC_STATIC, "exitWithoutEnter", "(Ljava/lang/Object;)V", null, null);
        mv.visitCode();
        mv.visitVarInsn(Opcodes.ALOAD, 0);
        mv.visitInsn(Opcodes.MONITOREXIT);
        mv.visitInsn(Opcodes.RETURN);
        mv.visitMaxs(1, 1);
        mv.visitEnd();

        // Generate `public static void main(String[])`:
        // try { exitWithoutEnter(new Object()); println("missing exception"); }
        // catch (IllegalMonitorStateException e) { println(e); }
        mv = cw.visitMethod(Opcodes.ACC_PUBLIC | Opcodes.ACC_STATIC, "main", "([Ljava/lang/String;)V", null, null);
        mv.visitCode();
        Label start = new Label();
        Label end = new Label();
        Label handler = new Label();
        Label done = new Label();
        mv.visitTryCatchBlock(start, end, handler, "java/lang/IllegalMonitorStateException");

        mv.visitLabel(start);
        mv.visitTypeInsn(Opcodes.NEW, "java/lang/Object");
        mv.visitInsn(Opcodes.DUP);
        mv.visitMethodInsn(Opcodes.INVOKESPECIAL, "java/lang/Object", "<init>", "()V", false);
        mv.visitMethodInsn(Opcodes.INVOKESTATIC, FULL_NAME, "exitWithoutEnter", "(Ljava/lang/Object;)V", false);
        mv.visitFieldInsn(Opcodes.GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
        mv.visitLdcInsn("missing exception");
        mv.visitMethodInsn(Opcodes.INVOKEVIRTUAL, "java/io/PrintStream", "println", "(Ljava/lang/String;)V", false);
        mv.visitLabel(end);
        mv.visitJumpInsn(Opcodes.GOTO, done);

        mv.visitLabel(handler); // the exception is on the stack
        mv.visitFieldInsn(Opcodes.GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
        mv.visitInsn(Opcodes.SWAP);
        mv.visitMethodInsn(Opcodes.INVOKEVIRTUAL, "java/io/PrintStream", "println", "(Ljava/lang/Object;)V", false);

        mv.visitLabel(done);
        mv.visitInsn(Opcodes.RETURN);
        mv.visitMaxs(2, 1);
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
