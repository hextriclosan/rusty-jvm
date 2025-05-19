# Dumping of generated class files

### Dumping properties

 - `-Djdk.invoke.LambdaMetafactory.dumpProxyClassFiles` - dump the lambda proxy classes
 - `-Djdk.invoke.MethodHandle.dumpClassFiles`
 - `-Djdk.invoke.MethodHandle.dumpHiddenClassFiles` - a default dumper for writing class files passed to Lookup::defineClass and Lookup::defineHiddenClass to disk for debugging purposes
 - `-Djdk.invoke.MethodHandle.dumpMethodHandleInternals`
 - `-Djdk.invoke.MethodHandleProxies.dumpClassFiles`

```bash
cargo run -- -Djdk.invoke.LambdaMetafactory.dumpProxyClassFiles -Djdk.invoke.MethodHandle.dumpClassFiles -Djdk.invoke.MethodHandle.dumpHiddenClassFiles -Djdk.invoke.MethodHandle.dumpMethodHandleInternals -Djdk.invoke.MethodHandleProxies.dumpClassFiles samples.reflection.methodhandleexample.MethodHandleExample
```
