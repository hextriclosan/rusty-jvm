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

final class VarHandleSegmentAsDoubles extends VarHandleSegmentViewBase {

    static final boolean BE = UNSAFE.isBigEndian();

    static final ScopedMemoryAccess SCOPED_MEMORY_ACCESS = ScopedMemoryAccess.getScopedMemoryAccess();

    static final int NON_PLAIN_ACCESS_MIN_ALIGN_MASK = Double.BYTES - 1;

    static final VarForm FORM = new VarForm(VarHandleSegmentAsDoubles.class, MemorySegment.class, double.class, long.class);

    VarHandleSegmentAsDoubles(boolean be, long alignmentMask, boolean exact) {
        super(FORM, be, alignmentMask, exact);
    }

    @Override
    final MethodType accessModeTypeUncached(VarHandle.AccessType accessType) {
        return accessType.accessModeType(MemorySegment.class, double.class, long.class);
    }

    @Override
    public VarHandleSegmentAsDoubles withInvokeExactBehavior() {
        return hasInvokeExactBehavior() ?
                this :
                new VarHandleSegmentAsDoubles(be, alignmentMask, true);
    }

    @Override
    public VarHandleSegmentAsDoubles withInvokeBehavior() {
        return !hasInvokeExactBehavior() ?
                this :
                new VarHandleSegmentAsDoubles(be, alignmentMask, false);
    }

    @ForceInline
    static long convEndian(boolean big, double v) {
        long rv = Double.doubleToRawLongBits(v);
        return big == BE ? rv : Long.reverseBytes(rv);
    }

    @ForceInline
    static double convEndian(boolean big, long rv) {
        rv = big == BE ? rv : Long.reverseBytes(rv);
        return Double.longBitsToDouble(rv);
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
    static double get(VarHandle ob, Object obb, long base) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, true);
        long rawValue = SCOPED_MEMORY_ACCESS.getLongUnaligned(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetPlain(bb, base),
                handle.be);
        return Double.longBitsToDouble(rawValue);
    }

    @ForceInline
    static void set(VarHandle ob, Object obb, long base, double value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        SCOPED_MEMORY_ACCESS.putLongUnaligned(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetPlain(bb, base),
                Double.doubleToRawLongBits(value),
                handle.be);
    }

    @ForceInline
    static double getVolatile(VarHandle ob, Object obb, long base) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, true);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.getLongVolatile(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask)));
    }

    @ForceInline
    static void setVolatile(VarHandle ob, Object obb, long base, double value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        SCOPED_MEMORY_ACCESS.putLongVolatile(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, value));
    }

    @ForceInline
    static double getAcquire(VarHandle ob, Object obb, long base) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, true);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.getLongAcquire(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask)));
    }

    @ForceInline
    static void setRelease(VarHandle ob, Object obb, long base, double value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        SCOPED_MEMORY_ACCESS.putLongRelease(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, value));
    }

    @ForceInline
    static double getOpaque(VarHandle ob, Object obb, long base) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, true);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.getLongOpaque(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask)));
    }

    @ForceInline
    static void setOpaque(VarHandle ob, Object obb, long base, double value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        SCOPED_MEMORY_ACCESS.putLongOpaque(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, value));
    }

    @ForceInline
    static boolean compareAndSet(VarHandle ob, Object obb, long base, double expected, double value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return SCOPED_MEMORY_ACCESS.compareAndSetLong(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, expected), convEndian(handle.be, value));
    }

    @ForceInline
    static double compareAndExchange(VarHandle ob, Object obb, long base, double expected, double value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.compareAndExchangeLong(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask),
                                  convEndian(handle.be, expected), convEndian(handle.be, value)));
    }

    @ForceInline
    static double compareAndExchangeAcquire(VarHandle ob, Object obb, long base, double expected, double value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.compareAndExchangeLongAcquire(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask),
                                  convEndian(handle.be, expected), convEndian(handle.be, value)));
    }

    @ForceInline
    static double compareAndExchangeRelease(VarHandle ob, Object obb, long base, double expected, double value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.compareAndExchangeLongRelease(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask),
                                  convEndian(handle.be, expected), convEndian(handle.be, value)));
    }

    @ForceInline
    static boolean weakCompareAndSetPlain(VarHandle ob, Object obb, long base, double expected, double value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return SCOPED_MEMORY_ACCESS.weakCompareAndSetLongPlain(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, expected), convEndian(handle.be, value));
    }

    @ForceInline
    static boolean weakCompareAndSet(VarHandle ob, Object obb, long base, double expected, double value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return SCOPED_MEMORY_ACCESS.weakCompareAndSetLong(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, expected), convEndian(handle.be, value));
    }

    @ForceInline
    static boolean weakCompareAndSetAcquire(VarHandle ob, Object obb, long base, double expected, double value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return SCOPED_MEMORY_ACCESS.weakCompareAndSetLongAcquire(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, expected), convEndian(handle.be, value));
    }

    @ForceInline
    static boolean weakCompareAndSetRelease(VarHandle ob, Object obb, long base, double expected, double value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return SCOPED_MEMORY_ACCESS.weakCompareAndSetLongRelease(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, expected), convEndian(handle.be, value));
    }

    @ForceInline
    static double getAndSet(VarHandle ob, Object obb, long base, double value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.getAndSetLong(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask),
                                  convEndian(handle.be, value)));
    }

    @ForceInline
    static double getAndSetAcquire(VarHandle ob, Object obb, long base, double value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.getAndSetLongAcquire(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask),
                                  convEndian(handle.be, value)));
    }

    @ForceInline
    static double getAndSetRelease(VarHandle ob, Object obb, long base, double value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.getAndSetLongRelease(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask),
                                  convEndian(handle.be, value)));
    }
}
