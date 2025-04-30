/*
 * Copyright (c) 2008, 2023, Oracle and/or its affiliates. All rights reserved.
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

// This file was previously auto-generated but is now manually maintained. Manual edits are allowed.

package sun.nio.fs;

import jdk.internal.util.OS;
import static java.lang.Integer.MIN_VALUE;

class UnixConstants {
    private UnixConstants() {
    }

    static final int O_RDONLY;
    static final int O_WRONLY;
    static final int O_RDWR;
    static final int O_APPEND;
    static final int O_CREAT;
    static final int O_EXCL;
    static final int O_TRUNC;
    static final int O_SYNC;

    static final int O_DSYNC;

    static final int O_NOFOLLOW;

    static final int O_DIRECT;

    static final int S_IAMB;
    static final int S_IRUSR;
    static final int S_IWUSR;
    static final int S_IXUSR;
    static final int S_IRGRP;
    static final int S_IWGRP;
    static final int S_IXGRP;
    static final int S_IROTH;
    static final int S_IWOTH;
    static final int S_IXOTH;

    static final int S_IFMT;
    static final int S_IFREG;
    static final int S_IFDIR;
    static final int S_IFLNK;
    static final int S_IFCHR;
    static final int S_IFBLK;
    static final int S_IFIFO;
    static final int R_OK;
    static final int W_OK;
    static final int X_OK;
    static final int F_OK;
    static final int ENOENT;
    static final int ENXIO;
    static final int EACCES;
    static final int EEXIST;
    static final int ENOTDIR;
    static final int EINVAL;
    static final int EXDEV;
    static final int EISDIR;
    static final int ENOTEMPTY;
    static final int ENOSPC;
    static final int EAGAIN;
    static final int EWOULDBLOCK;
    static final int ENOSYS;
    static final int ELOOP;
    static final int EROFS;

    static final int ENODATA;

    static final int XATTR_NOT_FOUND;

    static final int ERANGE;
    static final int EMFILE;

    static final int ENOTSUP;

    static final int AT_SYMLINK_NOFOLLOW;
    static final int AT_REMOVEDIR;

    static final int CLONE_NOFOLLOW;
    static final int CLONE_NOOWNERCOPY;

    static final int ATTR_CMN_CRTIME;
    static final int ATTR_CMN_MODTIME;
    static final int ATTR_CMN_ACCTIME;
    static final int FSOPT_NOFOLLOW;

    static final int POSIX_FADV_SEQUENTIAL;
    static final int POSIX_FADV_NOREUSE;
    static final int POSIX_FADV_WILLNEED;

    static {
        O_RDONLY = 00;
        O_WRONLY = 01;
        O_RDWR = 02;
        O_APPEND = OS.LINUX ? 02000 : 0x00000008;
        O_CREAT = OS.LINUX ? 0100 : 0x00000200;
        O_EXCL = OS.LINUX ? 0200 : 0x00000800;
        O_TRUNC = OS.LINUX ? 01000 : 0x00000400;
        O_SYNC = OS.LINUX ? 04010000 : 0x0080;

        O_DSYNC = OS.LINUX ? 010000 : 0x400000;

        O_NOFOLLOW = OS.LINUX ? 0400000 : 0x00000100;

        O_DIRECT = OS.LINUX ? 040000 : 00;

        S_IAMB = OS.LINUX ? (0400 | 0200 | 0100 | (0400 >> 3) | (0200 >> 3) | (0100 >> 3) | ((0400 >> 3) >> 3) | ((0200 >> 3) >> 3) | ((0100 >> 3) >> 3)) : (0000400 | 0000200 | 0000100 | 0000040 | 0000020 | 0000010 | 0000004 | 0000002 | 0000001);
        S_IRUSR = OS.LINUX ? 0400 : 0000400;
        S_IWUSR = OS.LINUX ? 0200 : 0000200;
        S_IXUSR = OS.LINUX ? 0100 : 0000100;
        S_IRGRP = OS.LINUX ? (0400 >> 3) : 0000040;
        S_IWGRP = OS.LINUX ? (0200 >> 3) : 0000020;
        S_IXGRP = OS.LINUX ? (0100 >> 3) : 0000010;
        S_IROTH = OS.LINUX ? ((0400 >> 3) >> 3) : 0000004;
        S_IWOTH = OS.LINUX ? ((0200 >> 3) >> 3) : 0000002;
        S_IXOTH = OS.LINUX ? ((0100 >> 3) >> 3) : 0000001;

        S_IFMT = OS.LINUX ? 0170000 : 0170000;
        S_IFREG = OS.LINUX ? 0100000 : 0100000;
        S_IFDIR = OS.LINUX ? 0040000 : 0040000;
        S_IFLNK = OS.LINUX ? 0120000 : 0120000;
        S_IFCHR = OS.LINUX ? 0020000 : 0020000;
        S_IFBLK = OS.LINUX ? 0060000 : 0060000;
        S_IFIFO = OS.LINUX ? 0010000 : 0010000;
        R_OK = OS.LINUX ? 4 : (1 << 2);
        W_OK = OS.LINUX ? 2 : (1 << 1);
        X_OK = OS.LINUX ? 1 : (1 << 0);
        F_OK = OS.LINUX ? 0 : 0;
        ENOENT = OS.LINUX ? 2 : 2;
        ENXIO = OS.LINUX ? 6 : 6;
        EACCES = OS.LINUX ? 13 : 13;
        EEXIST = OS.LINUX ? 17 : 17;
        ENOTDIR = 20;
        EINVAL = 22;
        EXDEV = 18;
        EISDIR = 21;
        ENOTEMPTY = OS.LINUX ? 39 : 66;
        ENOSPC = 28;
        EAGAIN = OS.LINUX ? 11 : 35;
        EWOULDBLOCK = OS.LINUX ? 11 : 35;
        ENOSYS = OS.LINUX ? 38 : 78;
        ELOOP = OS.LINUX ? 40 : 62;
        EROFS = 30;

        ENODATA = OS.LINUX ? 61 : 96;

        XATTR_NOT_FOUND = OS.LINUX ? 61 : 93;

        ERANGE = 34;
        EMFILE = 24;

        ENOTSUP = OS.LINUX ? MIN_VALUE : 45;

        AT_SYMLINK_NOFOLLOW = OS.LINUX ? 0x100 : 0x0020;
        AT_REMOVEDIR = OS.LINUX ? 0x200 : 0x0080;

        CLONE_NOFOLLOW = OS.LINUX ? MIN_VALUE : 0x0001;
        CLONE_NOOWNERCOPY = OS.LINUX ? MIN_VALUE : 0x0002;

        ATTR_CMN_CRTIME = OS.LINUX ? 00 : 0x00000200;
        ATTR_CMN_MODTIME = OS.LINUX ? 00 : 0x00000400;
        ATTR_CMN_ACCTIME = OS.LINUX ? 00 : 0x00001000;
        FSOPT_NOFOLLOW = OS.LINUX ? 00 : 0x00000001;

        POSIX_FADV_SEQUENTIAL = OS.LINUX ? 2 : MIN_VALUE;
        POSIX_FADV_NOREUSE = OS.LINUX ? 5 : MIN_VALUE;
        POSIX_FADV_WILLNEED = OS.LINUX ? 3 : MIN_VALUE;
    }
}
