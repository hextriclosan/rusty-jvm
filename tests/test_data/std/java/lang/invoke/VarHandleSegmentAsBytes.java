/*
 * Copyright (c) 2019, 2024, Oracle and/or its affiliates. All rights reserved.
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
package java.lang.invoke;

import jdk.internal.foreign.AbstractMemorySegmentImpl;
import jdk.internal.misc.ScopedMemoryAccess;
import jdk.internal.vm.annotation.ForceInline;

import java.lang.foreign.MemorySegment;
import java.lang.ref.Reference;

import java.util.Objects;

import static java.lang.invoke.MethodHandleStatics.UNSAFE;

// -- This file was mechanically generated: Do not edit! -- //

final class VarHandleSegmentAsBytes extends VarHandleSegmentViewBase {

    static final boolean BE = UNSAFE.isBigEndian();

    static final ScopedMemoryAccess SCOPED_MEMORY_ACCESS = ScopedMemoryAccess.getScopedMemoryAccess();

    static final int NON_PLAIN_ACCESS_MIN_ALIGN_MASK = Byte.BYTES - 1;

    static final VarForm FORM = new VarForm(VarHandleSegmentAsBytes.class, MemorySegment.class, byte.class, long.class);

    VarHandleSegmentAsBytes(boolean be, long alignmentMask, boolean exact) {
        super(FORM, be, alignmentMask, exact);
    }

    @Override
    final MethodType accessModeTypeUncached(VarHandle.AccessType accessType) {
        return accessType.accessModeType(MemorySegment.class, byte.class, long.class);
    }

    @Override
    public VarHandleSegmentAsBytes withInvokeExactBehavior() {
        return hasInvokeExactBehavior() ?
                this :
                new VarHandleSegmentAsBytes(be, alignmentMask, true);
    }

    @Override
    public VarHandleSegmentAsBytes withInvokeBehavior() {
        return !hasInvokeExactBehavior() ?
                this :
                new VarHandleSegmentAsBytes(be, alignmentMask, false);
    }

    @ForceInline
    static byte convEndian(boolean big, byte n) {
        return n;
    }

    @ForceInline
    static AbstractMemorySegmentImpl checkReadOnly(Object obb, boolean ro) {
        AbstractMemorySegmentImpl oo = (AbstractMemorySegmentImpl)Objects.requireNonNull(obb);
        oo.checkReadOnly(ro);
        return oo;
    }

    @ForceInline
    static long offsetNonPlain(AbstractMemorySegmentImpl bb, long offset, long alignmentMask) {
        if ((alignmentMask & NON_PLAIN_ACCESS_MIN_ALIGN_MASK) != NON_PLAIN_ACCESS_MIN_ALIGN_MASK) {
            throw VarHandleSegmentViewBase.newUnsupportedAccessModeForAlignment(alignmentMask + 1);
        }
        return offsetPlain(bb, offset);
    }

    @ForceInline
    static long offsetPlain(AbstractMemorySegmentImpl bb, long offset) {
        long base = bb.unsafeGetOffset();
        return base + offset;
    }

    @ForceInline
    static byte get(VarHandle ob, Object obb, long base) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, true);
        return SCOPED_MEMORY_ACCESS.getByte(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetPlain(bb, base));
    }

    @ForceInline
    static void set(VarHandle ob, Object obb, long base, byte value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        SCOPED_MEMORY_ACCESS.putByte(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetPlain(bb, base),
                value);
    }

    @ForceInline
    static byte getVolatile(VarHandle ob, Object obb, long base) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, true);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.getByteVolatile(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask)));
    }

    @ForceInline
    static void setVolatile(VarHandle ob, Object obb, long base, byte value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        SCOPED_MEMORY_ACCESS.putByteVolatile(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, value));
    }

    @ForceInline
    static byte getAcquire(VarHandle ob, Object obb, long base) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, true);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.getByteAcquire(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask)));
    }

    @ForceInline
    static void setRelease(VarHandle ob, Object obb, long base, byte value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        SCOPED_MEMORY_ACCESS.putByteRelease(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, value));
    }

    @ForceInline
    static byte getOpaque(VarHandle ob, Object obb, long base) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, true);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.getByteOpaque(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask)));
    }

    @ForceInline
    static void setOpaque(VarHandle ob, Object obb, long base, byte value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        SCOPED_MEMORY_ACCESS.putByteOpaque(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, value));
    }
}
