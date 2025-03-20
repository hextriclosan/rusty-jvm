# Patch the Standard Library

In some cases, modifying the standard library can be useful for debugging, testing, or experimenting with internal JDK functionality. 
This guide explains how to patch the java.base module in a controlled manner.

### Navigate to the directory where the standard library files are stored:
```bash
cd <PROJECT_DIR>/lib
```

### Patch the `java.base` Module
```bash
javac --patch-module java.base=. -d . java/lang/invoke/BoundMethodHandle.java
```
