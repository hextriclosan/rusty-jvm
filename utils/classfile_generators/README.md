# classfile_generators

## What?
`classfile_generators` is a helper program for generating synthetic Java class files directly, bypassing the Java compilation step. It is designed for testing and debugging purposes and is built using the [ASM][asm-link] library.

## Why?
Some bytecode instructions are rarely emitted by the Java compiler, making them difficult to test. However, these instructions can still appear in class files - such as `DUP_X2` in [`java.lang.invoke.LambdaForm`][lambda-form-link]. This tool is for generating class files manually, ensuring full control over the bytecode for better testing and analysis.

[//]: # (Links and footnotes)
[asm-link]: https://asm.ow2.io
[lambda-form-link]: ../../lib/java/lang/invoke/LambdaForm.class
