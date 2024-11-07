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

final class VarHandleSegmentAsInts extends VarHandleSegmentViewBase {

    static final boolean BE = UNSAFE.isBigEndian();

    static final ScopedMemoryAccess SCOPED_MEMORY_ACCESS = ScopedMemoryAccess.getScopedMemoryAccess();

    static final int NON_PLAIN_ACCESS_MIN_ALIGN_MASK = Integer.BYTES - 1;

    static final VarForm FORM = new VarForm(VarHandleSegmentAsInts.class, MemorySegment.class, int.class, long.class);

    VarHandleSegmentAsInts(boolean be, long alignmentMask, boolean exact) {
        super(FORM, be, alignmentMask, exact);
    }

    @Override
    final MethodType accessModeTypeUncached(VarHandle.AccessType accessType) {
        return accessType.accessModeType(MemorySegment.class, int.class, long.class);
    }

    @Override
    public VarHandleSegmentAsInts withInvokeExactBehavior() {
        return hasInvokeExactBehavior() ?
                this :
                new VarHandleSegmentAsInts(be, alignmentMask, true);
    }

    @Override
    public VarHandleSegmentAsInts withInvokeBehavior() {
        return !hasInvokeExactBehavior() ?
                this :
                new VarHandleSegmentAsInts(be, alignmentMask, false);
    }

    @ForceInline
    static int convEndian(boolean big, int n) {
        return big == BE ? n : Integer.reverseBytes(n);
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
    static int get(VarHandle ob, Object obb, long base) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, true);
        return SCOPED_MEMORY_ACCESS.getIntUnaligned(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetPlain(bb, base),
                handle.be);
    }

    @ForceInline
    static void set(VarHandle ob, Object obb, long base, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        SCOPED_MEMORY_ACCESS.putIntUnaligned(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetPlain(bb, base),
                value,
                handle.be);
    }

    @ForceInline
    static int getVolatile(VarHandle ob, Object obb, long base) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, true);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.getIntVolatile(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask)));
    }

    @ForceInline
    static void setVolatile(VarHandle ob, Object obb, long base, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        SCOPED_MEMORY_ACCESS.putIntVolatile(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, value));
    }

    @ForceInline
    static int getAcquire(VarHandle ob, Object obb, long base) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, true);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.getIntAcquire(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask)));
    }

    @ForceInline
    static void setRelease(VarHandle ob, Object obb, long base, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        SCOPED_MEMORY_ACCESS.putIntRelease(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, value));
    }

    @ForceInline
    static int getOpaque(VarHandle ob, Object obb, long base) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, true);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.getIntOpaque(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask)));
    }

    @ForceInline
    static void setOpaque(VarHandle ob, Object obb, long base, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        SCOPED_MEMORY_ACCESS.putIntOpaque(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, value));
    }

    @ForceInline
    static boolean compareAndSet(VarHandle ob, Object obb, long base, int expected, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return SCOPED_MEMORY_ACCESS.compareAndSetInt(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, expected), convEndian(handle.be, value));
    }

    @ForceInline
    static int compareAndExchange(VarHandle ob, Object obb, long base, int expected, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.compareAndExchangeInt(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask),
                                  convEndian(handle.be, expected), convEndian(handle.be, value)));
    }

    @ForceInline
    static int compareAndExchangeAcquire(VarHandle ob, Object obb, long base, int expected, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.compareAndExchangeIntAcquire(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask),
                                  convEndian(handle.be, expected), convEndian(handle.be, value)));
    }

    @ForceInline
    static int compareAndExchangeRelease(VarHandle ob, Object obb, long base, int expected, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.compareAndExchangeIntRelease(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask),
                                  convEndian(handle.be, expected), convEndian(handle.be, value)));
    }

    @ForceInline
    static boolean weakCompareAndSetPlain(VarHandle ob, Object obb, long base, int expected, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return SCOPED_MEMORY_ACCESS.weakCompareAndSetIntPlain(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, expected), convEndian(handle.be, value));
    }

    @ForceInline
    static boolean weakCompareAndSet(VarHandle ob, Object obb, long base, int expected, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return SCOPED_MEMORY_ACCESS.weakCompareAndSetInt(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, expected), convEndian(handle.be, value));
    }

    @ForceInline
    static boolean weakCompareAndSetAcquire(VarHandle ob, Object obb, long base, int expected, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return SCOPED_MEMORY_ACCESS.weakCompareAndSetIntAcquire(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, expected), convEndian(handle.be, value));
    }

    @ForceInline
    static boolean weakCompareAndSetRelease(VarHandle ob, Object obb, long base, int expected, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return SCOPED_MEMORY_ACCESS.weakCompareAndSetIntRelease(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, expected), convEndian(handle.be, value));
    }

    @ForceInline
    static int getAndSet(VarHandle ob, Object obb, long base, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.getAndSetInt(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask),
                                  convEndian(handle.be, value)));
    }

    @ForceInline
    static int getAndSetAcquire(VarHandle ob, Object obb, long base, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.getAndSetIntAcquire(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask),
                                  convEndian(handle.be, value)));
    }

    @ForceInline
    static int getAndSetRelease(VarHandle ob, Object obb, long base, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.getAndSetIntRelease(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask),
                                  convEndian(handle.be, value)));
    }

    @ForceInline
    static int getAndAdd(VarHandle ob, Object obb, long base, int delta) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndAddInt(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    delta);
        } else {
            return getAndAddConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), delta);
        }
    }

    @ForceInline
    static int getAndAddAcquire(VarHandle ob, Object obb, long base, int delta) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndAddIntAcquire(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    delta);
        } else {
            return getAndAddConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), delta);
        }
    }

    @ForceInline
    static int getAndAddRelease(VarHandle ob, Object obb, long base, int delta) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndAddIntRelease(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    delta);
        } else {
            return getAndAddConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), delta);
        }
    }

    @ForceInline
    static int getAndAddConvEndianWithCAS(AbstractMemorySegmentImpl  bb, long offset, int delta) {
        int nativeExpectedValue, expectedValue;
        Object base = bb.unsafeGetBase();
        do {
            nativeExpectedValue = SCOPED_MEMORY_ACCESS.getIntVolatile(bb.sessionImpl(),base, offset);
            expectedValue = Integer.reverseBytes(nativeExpectedValue);
        } while (!SCOPED_MEMORY_ACCESS.weakCompareAndSetInt(bb.sessionImpl(),base, offset,
                nativeExpectedValue, Integer.reverseBytes(expectedValue + delta)));
        return expectedValue;
    }

    @ForceInline
    static int getAndBitwiseOr(VarHandle ob, Object obb, long base, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndBitwiseOrInt(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    value);
        } else {
            return getAndBitwiseOrConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), value);
        }
    }

    @ForceInline
    static int getAndBitwiseOrRelease(VarHandle ob, Object obb, long base, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndBitwiseOrIntRelease(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    value);
        } else {
            return getAndBitwiseOrConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), value);
        }
    }

    @ForceInline
    static int getAndBitwiseOrAcquire(VarHandle ob, Object obb, long base, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndBitwiseOrIntAcquire(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    value);
        } else {
            return getAndBitwiseOrConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), value);
        }
    }

    @ForceInline
    static int getAndBitwiseOrConvEndianWithCAS(AbstractMemorySegmentImpl  bb, long offset, int value) {
        int nativeExpectedValue, expectedValue;
        Object base = bb.unsafeGetBase();
        do {
            nativeExpectedValue = SCOPED_MEMORY_ACCESS.getIntVolatile(bb.sessionImpl(),base, offset);
            expectedValue = Integer.reverseBytes(nativeExpectedValue);
        } while (!SCOPED_MEMORY_ACCESS.weakCompareAndSetInt(bb.sessionImpl(),base, offset,
                nativeExpectedValue, Integer.reverseBytes(expectedValue | value)));
        return expectedValue;
    }

    @ForceInline
    static int getAndBitwiseAnd(VarHandle ob, Object obb, long base, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndBitwiseAndInt(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    value);
        } else {
            return getAndBitwiseAndConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), value);
        }
    }

    @ForceInline
    static int getAndBitwiseAndRelease(VarHandle ob, Object obb, long base, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndBitwiseAndIntRelease(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    value);
        } else {
            return getAndBitwiseAndConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), value);
        }
    }

    @ForceInline
    static int getAndBitwiseAndAcquire(VarHandle ob, Object obb, long base, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndBitwiseAndIntAcquire(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    value);
        } else {
            return getAndBitwiseAndConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), value);
        }
    }

    @ForceInline
    static int getAndBitwiseAndConvEndianWithCAS(AbstractMemorySegmentImpl  bb, long offset, int value) {
        int nativeExpectedValue, expectedValue;
        Object base = bb.unsafeGetBase();
        do {
            nativeExpectedValue = SCOPED_MEMORY_ACCESS.getIntVolatile(bb.sessionImpl(),base, offset);
            expectedValue = Integer.reverseBytes(nativeExpectedValue);
        } while (!SCOPED_MEMORY_ACCESS.weakCompareAndSetInt(bb.sessionImpl(),base, offset,
                nativeExpectedValue, Integer.reverseBytes(expectedValue & value)));
        return expectedValue;
    }


    @ForceInline
    static int getAndBitwiseXor(VarHandle ob, Object obb, long base, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndBitwiseXorInt(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    value);
        } else {
            return getAndBitwiseXorConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), value);
        }
    }

    @ForceInline
    static int getAndBitwiseXorRelease(VarHandle ob, Object obb, long base, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndBitwiseXorIntRelease(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    value);
        } else {
            return getAndBitwiseXorConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), value);
        }
    }

    @ForceInline
    static int getAndBitwiseXorAcquire(VarHandle ob, Object obb, long base, int value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndBitwiseXorIntAcquire(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    value);
        } else {
            return getAndBitwiseXorConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), value);
        }
    }

    @ForceInline
    static int getAndBitwiseXorConvEndianWithCAS(AbstractMemorySegmentImpl  bb, long offset, int value) {
        int nativeExpectedValue, expectedValue;
        Object base = bb.unsafeGetBase();
        do {
            nativeExpectedValue = SCOPED_MEMORY_ACCESS.getIntVolatile(bb.sessionImpl(),base, offset);
            expectedValue = Integer.reverseBytes(nativeExpectedValue);
        } while (!SCOPED_MEMORY_ACCESS.weakCompareAndSetInt(bb.sessionImpl(),base, offset,
                nativeExpectedValue, Integer.reverseBytes(expectedValue ^ value)));
        return expectedValue;
    }
}
