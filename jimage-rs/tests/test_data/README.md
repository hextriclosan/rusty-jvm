### Crate Test Data

This directory contains test data for the `jimage-rs` crate.
The `java.base` module name is used for simplicity, as it is the only module without dependencies on other modules.

- `mods/java.base` - the test module packed into the jimage 1.0 files
- `mods_preview/java.base` - the same kind of module plus preview resources under `META-INF/preview/`, packed into the
  jimage 1.1 files

#### Compile the Test Module
To compile the test module:

```bash
javac --patch-module java.base=. -d mods --module-source-path src -m java.base
```
#### Build the jimage file
The generator writes `lib/modules`, built by the JDK under `JAVA_HOME` (JDK 25 or newer). Rename the result to the name of
the file it replaces.

```bash
cd ../../../utils/jimage_generators
./gradlew run --args="--endianness little"              # or big; add --compress for a compressed file
```

The jimage version follows the JDK: JDK 25-26 produce version 1.0, JDK 27+ produce version 1.1.
For the 1.1 files, build `mods_preview` with JDK 27 or newer:

```bash
./gradlew run --args="--endianness little --module-dir ../../jimage-rs/tests/test_data/mods_preview/java.base"
```
