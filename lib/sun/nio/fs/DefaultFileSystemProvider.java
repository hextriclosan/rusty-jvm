/*
 * Copyright (c) 2017, 2018, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.  Oracle designates this
 * particular file as subject to the "Classpath" exception as provided
 * by Oracle in the LICENSE file that accompanied this code.
 *
 * This code is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
 * version 2 for more details (a copy is included in the LICENSE file that
 * accompanied this code).
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA
 * or visit www.oracle.com if you need additional information or have any
 * questions.
 */

package sun.nio.fs;

import java.nio.file.FileSystem;
import java.nio.file.spi.FileSystemProvider;
import jdk.internal.util.OS;

/**
 * Creates this platform's default FileSystemProvider.
 */

public class DefaultFileSystemProvider {
    private static final FileSystemProvider INSTANCE;
    static {
        if (OS.WINDOWS) {
            INSTANCE = new WindowsFileSystemProvider();
        } else if (OS.LINUX) {
            INSTANCE = new LinuxFileSystemProvider();
        } else if (OS.MACOS) {
            INSTANCE = new MacOSXFileSystemProvider();
        } else {
            throw new UnsupportedOperationException("Unsupported operating system");
        }
    }

    private DefaultFileSystemProvider() { }

    /**
     * Returns the platform's default file system provider.
     */
    public static FileSystemProvider instance() {
        return INSTANCE;
    }

    /**
     * Returns the platform's default file system.
     */
    public static FileSystem theFileSystem() {
        if (INSTANCE instanceof WindowsFileSystemProvider win) {
            return win.theFileSystem();
        } else if (INSTANCE instanceof LinuxFileSystemProvider linux) {
            return linux.theFileSystem();
        } else if (INSTANCE instanceof MacOSXFileSystemProvider mac) {
            return mac.theFileSystem();
        } else {
            throw new UnsupportedOperationException("Unsupported file system provider: " + INSTANCE);
        }
    }
}
