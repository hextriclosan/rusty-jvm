
package jdk.internal.util;

public class OS { // todo: use OperatingSystem enum after fixing Native Call Error: Native method jdk/internal/reflect/DirectMethodHandleAccessor$NativeAccessor:invoke0:(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object; not found
    public static final boolean LINUX;
    public static final boolean MACOS;
    public static final boolean WINDOWS;

    static {
        String osName = System.getProperty("os.name").toLowerCase();
        LINUX = osName.contains("linux");
        MACOS = osName.contains("mac");
        WINDOWS = osName.contains("win");
    }

}
