# Tracing Method Handles in Rusty JVM

These properties are internal debugging flags for the OpenJDK's implementation of [JSR 292][Dynamically Typed Languages].

### Tracing properties

- `-Djava.lang.invoke.MethodHandle.DEBUG_NAMES`

Enables the generation of more descriptive, human-readable names for method handles and their internal LambdaForm representations. By default, these forms might have generic names like `LambdaForm$123`. When this flag is enabled, the names reflect the structure and purpose of the method handle, such as `MethodHandle(Test::main,String[]).asType(Object[])void`.

- `-Djava.lang.invoke.MethodHandle.TRACE_INTERPRETER`

Enables detailed, step-by-step tracing of method handle invocations as they are executed by the `java.lang.invoke` interpreter. The trace shows the "operand stack" for the method handle invocation, the operation being performed at each step (e.g., calling a target method, adapting an argument), and the intermediate results.

- `-Djava.lang.invoke.MethodHandle.TRACE_METHOD_LINKAGE`

Traces the dynamic linkage process for `invokedynamic` call sites. When an `invokedynamic` instruction is executed for the first time, the JVM calls a "bootstrap method" (BSM). This flag logs the invocation of the BSM, including the `Lookup` object, method name, method type, and any static arguments passed to it. It also shows the `CallSite` object that the BSM returns and the initial `MethodHandle` target it is linked to.

- `-Djava.lang.invoke.MethodHandle.TRACE_RESOLVE=true`

This helps debug the bridge between the JVM's internal method resolution and the `java.lang.invoke` API. If your Lookup implementation is failing to find a method or is resolving the wrong one, this trace will clarify what symbolic data is being used in the search, making it easier to debug access control checks and method lookup logic.

```bash
cargo run -- -Djava.lang.invoke.MethodHandle.DEBUG_NAMES=true -Djava.lang.invoke.MethodHandle.TRACE_INTERPRETER=true -Djava.lang.invoke.MethodHandle.TRACE_METHOD_LINKAGE=true -Djava.lang.invoke.MethodHandle.TRACE_RESOLVE=true samples.reflection.mutablecallsiteexample.MutableCallSiteExample
```

[//]: # (links)
[Dynamically Typed Languages]: https://www.oracle.com/technical-resources/articles/javase/dyntypelang.html
