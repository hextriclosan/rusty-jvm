package com.github.hextriclosan.rustyjvm;

import org.objectweb.asm.Attribute;
import org.objectweb.asm.ByteVector;
import org.objectweb.asm.ClassReader;
import org.objectweb.asm.ClassWriter;
import org.objectweb.asm.MethodVisitor;
import org.objectweb.asm.Opcodes;
import org.objectweb.asm.Type;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.util.Arrays;

import static com.github.hextriclosan.rustyjvm.Constants.JCLASSFILE_OUTPUT_PATH;

/**
 * Class files that fail format checking (JVMS §4.8), each of them on a single check: javac cannot
 * produce them, and HotSpot rejects every one with the {@code ClassFormatError} quoted next to it
 * ({@code java -Xverify:all -cp <output directory> <class name>}).
 * <p>
 * ASM does not look at names and descriptors, so a class file whose constant pool is malformed is
 * generated as it is. A class file that is malformed as a sequence of bytes is a well-formed one,
 * changed after it is generated.
 */
public class MalformedClassFileGenerator {
    private static final String OUTPUT_DIR = JCLASSFILE_OUTPUT_PATH + "/malformed";

    private static final String OBJECT = "java/lang/Object";
    private static final String FIELD_NAME = "someText";
    private static final String NO_ARGUMENTS = "()V";

    public static void main(String[] args) throws IOException {
        // The first four bytes must contain the right magic number: the last of them is 0xBF.
        // Incompatible magic value 3405691583 in class file WrongMagic
        byte[] wrongMagic = generate("WrongMagic", FIELD_NAME, OBJECT, NO_ARGUMENTS);
        wrongMagic[3] = (byte) 0xBF;
        write("WrongMagic", wrongMagic);

        // All predefined attributes must be of the proper length: SourceFile is three bytes long.
        // Wrong SourceFile attribute length in class file WrongAttributeLength
        write("WrongAttributeLength", generateWithSourceFileOfThreeBytes("WrongAttributeLength"));

        // The class file must not be truncated: the last byte is missing.
        // Truncated class file
        byte[] truncated = generate("Truncated", FIELD_NAME, OBJECT, NO_ARGUMENTS);
        write("Truncated", Arrays.copyOf(truncated, truncated.length - 1));

        // The class file must not have extra bytes at the end: two bytes follow the last attribute.
        // Extra bytes at the end of class file ExtraBytes
        byte[] extraBytes = generate("ExtraBytes", FIELD_NAME, OBJECT, NO_ARGUMENTS);
        write("ExtraBytes", Arrays.copyOf(extraBytes, extraBytes.length + 2));

        // The constant pool must satisfy the constraints of JVMS §4.4: a CONSTANT_MethodType_info
        // is in a class file of version 50.0, which does not define its tag.
        // Class file version does not support constant tag 16 in class file UnsupportedConstantTag
        write("UnsupportedConstantTag", generateWithMethodType("UnsupportedConstantTag", Opcodes.V1_6));

        // The constant pool must satisfy the constraints of JVMS §4.4: the CONSTANT_Utf8_info of
        // the field name starts with the byte 0xFF.
        // Illegal UTF8 string in constant pool in class file MalformedUtf8
        byte[] malformedUtf8 = generate("MalformedUtf8", FIELD_NAME, OBJECT, NO_ARGUMENTS);
        malformedUtf8[utf8BytesOffset(malformedUtf8, FIELD_NAME)] = (byte) 0xFF;
        write("MalformedUtf8", malformedUtf8);

        // Field and method references must have valid classes: the constructor of the superclass
        // is a method of java.lang/Object.
        // Illegal class name "java.lang/Object" in class file MalformedClassName
        write("MalformedClassName", generate("MalformedClassName", FIELD_NAME, "java.lang/Object", NO_ARGUMENTS));

        // Field and method references must have valid names: the field is named some.Text.
        // Illegal field name "some.Text" in class MalformedFieldName
        write("MalformedFieldName", generate("MalformedFieldName", "some.Text", OBJECT, NO_ARGUMENTS));

        // Field and method references must have valid descriptors: the descriptor of the constructor
        // of the superclass is (Ljava/lang/String:)V.
        // Method "<init>" in class MalformedMethodDescriptor has illegal signature "(Ljava/lang/String:)V"
        write("MalformedMethodDescriptor", generate("MalformedMethodDescriptor", FIELD_NAME, OBJECT, "(Ljava/lang/String:)V"));

        System.out.println("classes written successfully.");
    }

    /**
     * Generates
     * <pre>
     * public class {className} {
     *     private String {fieldName};
     *
     *     public {className}(String text) {
     *         super();
     *         this.{fieldName} = text;
     *     }
     * }
     * </pre>
     * where {@code super()} invokes {@code {superClass}.<init>{superConstructorDescriptor}}.
     */
    private static byte[] generate(String className, String fieldName, String superClass, String superConstructorDescriptor) {
        ClassWriter cw = new ClassWriter(0);
        cw.visit(Opcodes.V23, Opcodes.ACC_PUBLIC, className, null, OBJECT, null);
        cw.visitSource(className + ".java", null);

        cw.visitField(Opcodes.ACC_PRIVATE, fieldName, "Ljava/lang/String;", null, null).visitEnd();

        MethodVisitor mv = cw.visitMethod(Opcodes.ACC_PUBLIC, "<init>", "(Ljava/lang/String;)V", null, null);
        mv.visitCode();
        mv.visitVarInsn(Opcodes.ALOAD, 0);
        mv.visitMethodInsn(Opcodes.INVOKESPECIAL, superClass, "<init>", superConstructorDescriptor, false);
        mv.visitVarInsn(Opcodes.ALOAD, 0);
        mv.visitVarInsn(Opcodes.ALOAD, 1);
        mv.visitFieldInsn(Opcodes.PUTFIELD, className, fieldName, "Ljava/lang/String;");
        mv.visitInsn(Opcodes.RETURN);
        mv.visitMaxs(2, 2);
        mv.visitEnd();

        cw.visitEnd();

        return cw.toByteArray();
    }

    /**
     * Generates a class whose only member is {@code static Object methodType()}, which returns the
     * method type {@code ()V} that it loads from the constant pool.
     */
    private static byte[] generateWithMethodType(String className, int version) {
        ClassWriter cw = new ClassWriter(0);
        cw.visit(version, Opcodes.ACC_PUBLIC, className, null, OBJECT, null);

        MethodVisitor mv = cw.visitMethod(Opcodes.ACC_STATIC, "methodType", "()Ljava/lang/Object;", null, null);
        mv.visitCode();
        mv.visitLdcInsn(Type.getMethodType(NO_ARGUMENTS));
        mv.visitInsn(Opcodes.ARETURN);
        mv.visitMaxs(1, 0);
        mv.visitEnd();

        cw.visitEnd();

        return cw.toByteArray();
    }

    /**
     * Generates a class without members, whose {@code SourceFile} attribute has one byte after its
     * {@code sourcefile_index} item.
     */
    private static byte[] generateWithSourceFileOfThreeBytes(String className) {
        ClassWriter cw = new ClassWriter(0);
        cw.visit(Opcodes.V23, Opcodes.ACC_PUBLIC, className, null, OBJECT, null);

        cw.visitAttribute(new Attribute("SourceFile") {
            @Override
            protected ByteVector write(ClassWriter classWriter, byte[] code, int codeLength, int maxStack, int maxLocals) {
                return new ByteVector().putShort(classWriter.newUTF8(className + ".java")).putByte(0);
            }
        });

        cw.visitEnd();

        return cw.toByteArray();
    }

    /**
     * Finds the {@code bytes} item of the {@code CONSTANT_Utf8_info} structure that holds a string.
     */
    private static int utf8BytesOffset(byte[] classFile, String value) {
        byte[] bytes = value.getBytes(StandardCharsets.UTF_8);
        ClassReader reader = new ClassReader(classFile);

        for (int index = 1; index < reader.getItemCount(); index++) {
            // Just after the tag; zero for the entry that follows an 8-byte constant
            int offset = reader.getItem(index);
            if (offset != 0
                    && classFile[offset - 1] == 1 // CONSTANT_Utf8
                    && reader.readUnsignedShort(offset) == bytes.length
                    && Arrays.equals(classFile, offset + 2, offset + 2 + bytes.length, bytes, 0, bytes.length)) {
                return offset + 2;
            }
        }

        throw new IllegalArgumentException("No CONSTANT_Utf8_info holds " + value);
    }

    private static void write(String className, byte[] classFile) throws IOException {
        File outputDir = new File(OUTPUT_DIR);
        outputDir.mkdirs();
        try (FileOutputStream fos = new FileOutputStream(new File(outputDir, className + ".class"))) {
            fos.write(classFile);
        }
    }
}
