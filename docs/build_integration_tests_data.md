# Build Integration Tests Data

```shell
javac -d . $(find . -maxdepth 1 -name '*.java' ! -name 'StringConcatInline.java' ! -name 'BootstrapMethodInvokerExample.java' ! -name 'ConstantPoolExample.java' ! -name 'StringConcatHelperExample.java' ! -name 'ReflectionAreNestMatesExample.java' ! -name 'ReflectionGetCallerClassExample.java' ! -name 'UnsafeGetLongUnalignedExample.java' ! -name 'UnsafeUsage.java' ! -name 'UnsafeObjectFieldOffset.java' ! -name 'UnsafePutReferenceVolatileExample.java') \
&& javac -XDstringConcat=inline  -d . StringConcatInline.java \
&& javac --patch-module java.base=. -d . BootstrapMethodInvokerExample.java \
&& javac --add-exports java.base/jdk.internal.reflect=ALL-UNNAMED --patch-module java.base=. -d . ConstantPoolExample.java \
&& javac --patch-module java.base=. -d . StringConcatHelperExample.java \
&& javac --add-exports java.base/jdk.internal.reflect=ALL-UNNAMED -d . ReflectionAreNestMatesExample.java \
&& javac --add-exports java.base/jdk.internal.reflect=ALL-UNNAMED  -d . ReflectionGetCallerClassExample.java \
&& javac --add-exports java.base/jdk.internal.misc=ALL-UNNAMED -d . UnsafeGetLongUnalignedExample.java \
&& javac --add-exports java.base/jdk.internal.misc=ALL-UNNAMED -d . UnsafeUsage.java \
&& javac --add-exports java.base/jdk.internal.misc=ALL-UNNAMED  -d . UnsafeObjectFieldOffset.java \
&& javac --add-exports java.base/jdk.internal.misc=ALL-UNNAMED -d . UnsafePutReferenceVolatileExample.java
```
