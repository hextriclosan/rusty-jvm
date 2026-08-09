# Vendored test dependency

`algorithm.jar` is a third-party library used by two integration tests:

| Test | What it covers |
|---|---|
| `should_use_jar_from_classpath` | wildcard classpath expansion (`-cp ".:lib_jar/*"`) |
| `should_run_jar` | `-jar` mode plus the `Class-Path: lib_jar/algorithm.jar` manifest entry of `app.jar` |

It is also needed at *build* time: `build.rs` compiles `ClasspathDemo.java` and
`jar/src/samples/jarfiles/simplejar/Main.java` against it, then copies it to
`<target>/java_classes_for_tests/lib_jar/algorithm.jar`, which is where both tests expect it.
The file name must stay `algorithm.jar`, because `tests/test_data/jar/MANIFEST.MF` refers to it
by that path.

## Provenance

| | |
|---|---|
| Coordinates | `io.github.hextriclosan:algorithm:0.0.5` |
| Source | https://repo1.maven.org/maven2/io/github/hextriclosan/algorithm/0.0.5/algorithm-0.0.5.jar |
| SHA-256 | `31c2f4f9af4a60ba24cb15f3708f5a48db1671d5735288beb2259ecc986c1734` |
| Size | 17238 bytes |
