/*
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as published by
 * the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

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
