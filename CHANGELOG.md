# 0.5.0

## What's Changed
* Switch to java 25 (LTS) core library classes
* Improve exception handling
* Fixes for big-endian platforms


# 0.4.0

## What's Changed
* Native methods `System.nanoTime()`, `Class.getDeclaredMethod(...)`, `Class.getDeclaredConstructor(...)`, `Class.getMethods()`, `Class.getFields()`
* Support for `invokedynamic` instruction and `java.lang.invoke` package (including support for lambdas, stream API, records)
* Basic support of `java.util.zip` package


# 0.3.0

## What's Changed
* Integrated the `jimage-rs` crate for reading JDK image files
* Replaced bundled classes with core library classes from the installed JDK


# 0.2.1

## What's Changed
* Rework installation
* Polishing readme


# 0.2.0

## What's Changed
* Added support for opcodes: `nop`, `dup2_x1`, `dup2_x2`, `swap`, `frem`, `fneg`, `dneg`, `d2f`, `goto_w`
* Added option for installing and removing standard library classes
* Refactored Library API


# 0.1.0

## What's Added
Added support for
* nearly all JVM bytecode instructions
* static initialization
* polymorphism
* exceptions
* program arguments
* core standard library classes
