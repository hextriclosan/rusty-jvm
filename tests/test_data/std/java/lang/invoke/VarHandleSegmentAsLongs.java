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

final class VarHandleSegmentAsLongs extends VarHandleSegmentViewBase {

    static final boolean BE = UNSAFE.isBigEndian();

    static final ScopedMemoryAccess SCOPED_MEMORY_ACCESS = ScopedMemoryAccess.getScopedMemoryAccess();

    static final int NON_PLAIN_ACCESS_MIN_ALIGN_MASK = Long.BYTES - 1;

    static final VarForm FORM = new VarForm(VarHandleSegmentAsLongs.class, MemorySegment.class, long.class, long.class);

    VarHandleSegmentAsLongs(boolean be, long alignmentMask, boolean exact) {
        super(FORM, be, alignmentMask, exact);
    }

    @Override
    final MethodType accessModeTypeUncached(VarHandle.AccessType accessType) {
        return accessType.accessModeType(MemorySegment.class, long.class, long.class);
    }

    @Override
    public VarHandleSegmentAsLongs withInvokeExactBehavior() {
        return hasInvokeExactBehavior() ?
                this :
                new VarHandleSegmentAsLongs(be, alignmentMask, true);
    }

    @Override
    public VarHandleSegmentAsLongs withInvokeBehavior() {
        return !hasInvokeExactBehavior() ?
                this :
                new VarHandleSegmentAsLongs(be, alignmentMask, false);
    }

    @ForceInline
    static long convEndian(boolean big, long n) {
        return big == BE ? n : Long.reverseBytes(n);
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
    static long get(VarHandle ob, Object obb, long base) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, true);
        return SCOPED_MEMORY_ACCESS.getLongUnaligned(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetPlain(bb, base),
                handle.be);
    }

    @ForceInline
    static void set(VarHandle ob, Object obb, long base, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        SCOPED_MEMORY_ACCESS.putLongUnaligned(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetPlain(bb, base),
                value,
                handle.be);
    }

    @ForceInline
    static long getVolatile(VarHandle ob, Object obb, long base) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, true);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.getLongVolatile(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask)));
    }

    @ForceInline
    static void setVolatile(VarHandle ob, Object obb, long base, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        SCOPED_MEMORY_ACCESS.putLongVolatile(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, value));
    }

    @ForceInline
    static long getAcquire(VarHandle ob, Object obb, long base) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, true);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.getLongAcquire(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask)));
    }

    @ForceInline
    static void setRelease(VarHandle ob, Object obb, long base, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        SCOPED_MEMORY_ACCESS.putLongRelease(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, value));
    }

    @ForceInline
    static long getOpaque(VarHandle ob, Object obb, long base) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, true);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.getLongOpaque(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask)));
    }

    @ForceInline
    static void setOpaque(VarHandle ob, Object obb, long base, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        SCOPED_MEMORY_ACCESS.putLongOpaque(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, value));
    }

    @ForceInline
    static boolean compareAndSet(VarHandle ob, Object obb, long base, long expected, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return SCOPED_MEMORY_ACCESS.compareAndSetLong(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, expected), convEndian(handle.be, value));
    }

    @ForceInline
    static long compareAndExchange(VarHandle ob, Object obb, long base, long expected, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.compareAndExchangeLong(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask),
                                  convEndian(handle.be, expected), convEndian(handle.be, value)));
    }

    @ForceInline
    static long compareAndExchangeAcquire(VarHandle ob, Object obb, long base, long expected, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.compareAndExchangeLongAcquire(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask),
                                  convEndian(handle.be, expected), convEndian(handle.be, value)));
    }

    @ForceInline
    static long compareAndExchangeRelease(VarHandle ob, Object obb, long base, long expected, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.compareAndExchangeLongRelease(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask),
                                  convEndian(handle.be, expected), convEndian(handle.be, value)));
    }

    @ForceInline
    static boolean weakCompareAndSetPlain(VarHandle ob, Object obb, long base, long expected, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return SCOPED_MEMORY_ACCESS.weakCompareAndSetLongPlain(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, expected), convEndian(handle.be, value));
    }

    @ForceInline
    static boolean weakCompareAndSet(VarHandle ob, Object obb, long base, long expected, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return SCOPED_MEMORY_ACCESS.weakCompareAndSetLong(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, expected), convEndian(handle.be, value));
    }

    @ForceInline
    static boolean weakCompareAndSetAcquire(VarHandle ob, Object obb, long base, long expected, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return SCOPED_MEMORY_ACCESS.weakCompareAndSetLongAcquire(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, expected), convEndian(handle.be, value));
    }

    @ForceInline
    static boolean weakCompareAndSetRelease(VarHandle ob, Object obb, long base, long expected, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return SCOPED_MEMORY_ACCESS.weakCompareAndSetLongRelease(bb.sessionImpl(),
                bb.unsafeGetBase(),
                offsetNonPlain(bb, base, handle.alignmentMask),
                convEndian(handle.be, expected), convEndian(handle.be, value));
    }

    @ForceInline
    static long getAndSet(VarHandle ob, Object obb, long base, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.getAndSetLong(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask),
                                  convEndian(handle.be, value)));
    }

    @ForceInline
    static long getAndSetAcquire(VarHandle ob, Object obb, long base, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.getAndSetLongAcquire(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask),
                                  convEndian(handle.be, value)));
    }

    @ForceInline
    static long getAndSetRelease(VarHandle ob, Object obb, long base, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        return convEndian(handle.be,
                          SCOPED_MEMORY_ACCESS.getAndSetLongRelease(bb.sessionImpl(),
                                  bb.unsafeGetBase(),
                                  offsetNonPlain(bb, base, handle.alignmentMask),
                                  convEndian(handle.be, value)));
    }

    @ForceInline
    static long getAndAdd(VarHandle ob, Object obb, long base, long delta) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndAddLong(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    delta);
        } else {
            return getAndAddConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), delta);
        }
    }

    @ForceInline
    static long getAndAddAcquire(VarHandle ob, Object obb, long base, long delta) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndAddLongAcquire(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    delta);
        } else {
            return getAndAddConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), delta);
        }
    }

    @ForceInline
    static long getAndAddRelease(VarHandle ob, Object obb, long base, long delta) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndAddLongRelease(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    delta);
        } else {
            return getAndAddConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), delta);
        }
    }

    @ForceInline
    static long getAndAddConvEndianWithCAS(AbstractMemorySegmentImpl  bb, long offset, long delta) {
        long nativeExpectedValue, expectedValue;
        Object base = bb.unsafeGetBase();
        do {
            nativeExpectedValue = SCOPED_MEMORY_ACCESS.getLongVolatile(bb.sessionImpl(),base, offset);
            expectedValue = Long.reverseBytes(nativeExpectedValue);
        } while (!SCOPED_MEMORY_ACCESS.weakCompareAndSetLong(bb.sessionImpl(),base, offset,
                nativeExpectedValue, Long.reverseBytes(expectedValue + delta)));
        return expectedValue;
    }

    @ForceInline
    static long getAndBitwiseOr(VarHandle ob, Object obb, long base, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndBitwiseOrLong(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    value);
        } else {
            return getAndBitwiseOrConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), value);
        }
    }

    @ForceInline
    static long getAndBitwiseOrRelease(VarHandle ob, Object obb, long base, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndBitwiseOrLongRelease(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    value);
        } else {
            return getAndBitwiseOrConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), value);
        }
    }

    @ForceInline
    static long getAndBitwiseOrAcquire(VarHandle ob, Object obb, long base, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndBitwiseOrLongAcquire(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    value);
        } else {
            return getAndBitwiseOrConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), value);
        }
    }

    @ForceInline
    static long getAndBitwiseOrConvEndianWithCAS(AbstractMemorySegmentImpl  bb, long offset, long value) {
        long nativeExpectedValue, expectedValue;
        Object base = bb.unsafeGetBase();
        do {
            nativeExpectedValue = SCOPED_MEMORY_ACCESS.getLongVolatile(bb.sessionImpl(),base, offset);
            expectedValue = Long.reverseBytes(nativeExpectedValue);
        } while (!SCOPED_MEMORY_ACCESS.weakCompareAndSetLong(bb.sessionImpl(),base, offset,
                nativeExpectedValue, Long.reverseBytes(expectedValue | value)));
        return expectedValue;
    }

    @ForceInline
    static long getAndBitwiseAnd(VarHandle ob, Object obb, long base, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndBitwiseAndLong(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    value);
        } else {
            return getAndBitwiseAndConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), value);
        }
    }

    @ForceInline
    static long getAndBitwiseAndRelease(VarHandle ob, Object obb, long base, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndBitwiseAndLongRelease(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    value);
        } else {
            return getAndBitwiseAndConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), value);
        }
    }

    @ForceInline
    static long getAndBitwiseAndAcquire(VarHandle ob, Object obb, long base, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndBitwiseAndLongAcquire(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    value);
        } else {
            return getAndBitwiseAndConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), value);
        }
    }

    @ForceInline
    static long getAndBitwiseAndConvEndianWithCAS(AbstractMemorySegmentImpl  bb, long offset, long value) {
        long nativeExpectedValue, expectedValue;
        Object base = bb.unsafeGetBase();
        do {
            nativeExpectedValue = SCOPED_MEMORY_ACCESS.getLongVolatile(bb.sessionImpl(),base, offset);
            expectedValue = Long.reverseBytes(nativeExpectedValue);
        } while (!SCOPED_MEMORY_ACCESS.weakCompareAndSetLong(bb.sessionImpl(),base, offset,
                nativeExpectedValue, Long.reverseBytes(expectedValue & value)));
        return expectedValue;
    }


    @ForceInline
    static long getAndBitwiseXor(VarHandle ob, Object obb, long base, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndBitwiseXorLong(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    value);
        } else {
            return getAndBitwiseXorConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), value);
        }
    }

    @ForceInline
    static long getAndBitwiseXorRelease(VarHandle ob, Object obb, long base, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndBitwiseXorLongRelease(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    value);
        } else {
            return getAndBitwiseXorConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), value);
        }
    }

    @ForceInline
    static long getAndBitwiseXorAcquire(VarHandle ob, Object obb, long base, long value) {
        VarHandleSegmentViewBase handle = (VarHandleSegmentViewBase)ob;
        AbstractMemorySegmentImpl bb = checkReadOnly(obb, false);
        if (handle.be == BE) {
            return SCOPED_MEMORY_ACCESS.getAndBitwiseXorLongAcquire(bb.sessionImpl(),
                    bb.unsafeGetBase(),
                    offsetNonPlain(bb, base, handle.alignmentMask),
                    value);
        } else {
            return getAndBitwiseXorConvEndianWithCAS(bb, offsetNonPlain(bb, base, handle.alignmentMask), value);
        }
    }

    @ForceInline
    static long getAndBitwiseXorConvEndianWithCAS(AbstractMemorySegmentImpl  bb, long offset, long value) {
        long nativeExpectedValue, expectedValue;
        Object base = bb.unsafeGetBase();
        do {
            nativeExpectedValue = SCOPED_MEMORY_ACCESS.getLongVolatile(bb.sessionImpl(),base, offset);
            expectedValue = Long.reverseBytes(nativeExpectedValue);
        } while (!SCOPED_MEMORY_ACCESS.weakCompareAndSetLong(bb.sessionImpl(),base, offset,
                nativeExpectedValue, Long.reverseBytes(expectedValue ^ value)));
        return expectedValue;
    }
}
