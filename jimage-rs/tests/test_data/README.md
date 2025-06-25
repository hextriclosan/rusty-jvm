### Crate Test Data

This directory contains test data for the `jimage-rs` crate.
The `java.base` module name is used for simplicity, as it is the only module without dependencies on other modules.

#### Compile the Test Module
To compile the test module:

```bash
javac --patch-module java.base=. -d mods --module-source-path src -m java.base
```
#### Build the jimage file
To build the `lib/modules` file:

```bash
cd ../../../utils/jimage_generators
./gradlew run
```
