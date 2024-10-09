/*
 * Copyright (c) 2015, 2022, Oracle and/or its affiliates. All rights reserved.
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

import jdk.internal.access.JavaNioAccess;
import jdk.internal.access.SharedSecrets;
import jdk.internal.foreign.AbstractMemorySegmentImpl;
import jdk.internal.foreign.MemorySessionImpl;
import jdk.internal.misc.ScopedMemoryAccess;
import jdk.internal.misc.Unsafe;
import jdk.internal.util.Preconditions;
import jdk.internal.vm.annotation.ForceInline;

import java.lang.foreign.MemorySegment;
import java.nio.ByteBuffer;
import java.nio.ReadOnlyBufferException;
import java.util.List;
import java.util.Objects;
import java.util.function.BiFunction;

import static java.lang.invoke.MethodHandleStatics.UNSAFE;

// -- This file was mechanically generated: Do not edit! -- //

final class VarHandleByteArrayAsLongs extends VarHandleByteArrayBase {

    static final JavaNioAccess NIO_ACCESS = SharedSecrets.getJavaNioAccess();

    static final int ALIGN = Long.BYTES - 1;

    static final ScopedMemoryAccess SCOPED_MEMORY_ACCESS = ScopedMemoryAccess.getScopedMemoryAccess();

    @ForceInline
    static long convEndian(boolean big, long n) {
        return big == BE ? n : Long.reverseBytes(n);
    }


    static abstract sealed class ByteArrayViewVarHandle extends VarHandle {
        final boolean be;

        ByteArrayViewVarHandle(VarForm form, boolean be, boolean exact) {
            super(form, exact);
            this.be = be;
        }
    }

    static final class ArrayHandle extends ByteArrayViewVarHandle {

        ArrayHandle(boolean be) {
            this(be, false);
        }

        private ArrayHandle(boolean be, boolean exact) {
            super(ArrayHandle.FORM, be, exact);
        }

        @Override
        public ArrayHandle withInvokeExactBehavior() {
            return hasInvokeExactBehavior()
                ? this
                : new ArrayHandle(be, true);
        }

        @Override
        public ArrayHandle withInvokeBehavior() {
            return !hasInvokeExactBehavior()
                ? this
                : new ArrayHandle(be, false);
        }

        @Override
        final MethodType accessModeTypeUncached(AccessType at) {
            return at.accessModeType(byte[].class, long.class, int.class);
        }

        @ForceInline
        static int index(byte[] ba, int index) {
            return Preconditions.checkIndex(index, ba.length - ALIGN, Preconditions.AIOOBE_FORMATTER);
        }

        @ForceInline
        static long get(VarHandle ob, Object oba, int index) {
            ArrayHandle handle = (ArrayHandle)ob;
            byte[] ba = (byte[]) oba;
            return UNSAFE.getLongUnaligned(
                    ba,
                    ((long) index(ba, index)) + Unsafe.ARRAY_BYTE_BASE_OFFSET,
                    handle.be);
        }

        @ForceInline
        static void set(VarHandle ob, Object oba, int index, long value) {
            ArrayHandle handle = (ArrayHandle)ob;
            byte[] ba = (byte[]) oba;
            UNSAFE.putLongUnaligned(
                    ba,
                    ((long) index(ba, index)) + Unsafe.ARRAY_BYTE_BASE_OFFSET,
                    value,
                    handle.be);
        }

        static final VarForm FORM = new VarForm(ArrayHandle.class, byte[].class, long.class, int.class);
    }


    static final class ByteBufferHandle extends ByteArrayViewVarHandle {

        ByteBufferHandle(boolean be) {
            this(be, false);
        }

        private ByteBufferHandle(boolean be, boolean exact) {
            super(ByteBufferHandle.FORM, be, exact);
        }

        @Override
        public ByteBufferHandle withInvokeExactBehavior() {
            return hasInvokeExactBehavior()
                ? this
                : new ByteBufferHandle(be, true);
        }

        @Override
        public ByteBufferHandle withInvokeBehavior() {
            return !hasInvokeExactBehavior()
                ? this
                : new ByteBufferHandle(be, false);
        }

        @Override
        final MethodType accessModeTypeUncached(AccessType at) {
            return at.accessModeType(ByteBuffer.class, long.class, int.class);
        }

        @ForceInline
        static int index(ByteBuffer bb, int index) {
            MemorySegment segment = NIO_ACCESS.bufferSegment(bb);
            return Preconditions.checkIndex(index, UNSAFE.getInt(bb, BUFFER_LIMIT) - ALIGN, null);
        }

        @ForceInline
        static MemorySessionImpl session(ByteBuffer bb) {
            MemorySegment segment = NIO_ACCESS.bufferSegment(bb);
            return segment != null ?
                    ((AbstractMemorySegmentImpl)segment).sessionImpl() : null;
        }

        @ForceInline
        static int indexRO(ByteBuffer bb, int index) {
            if (UNSAFE.getBoolean(bb, BYTE_BUFFER_IS_READ_ONLY))
                throw new ReadOnlyBufferException();
            return index(bb, index);
        }

        @ForceInline
        static long address(ByteBuffer bb, int index) {
            long address = ((long) index) + UNSAFE.getLong(bb, BUFFER_ADDRESS);
            if ((address & ALIGN) != 0)
                throw newIllegalStateExceptionForMisalignedAccess(index);
            return address;
        }

        @ForceInline
        static Object checkNullHeapBase(Object hb) {
            if (hb != null) {
                throw new IllegalStateException("Atomic access not supported for heap buffer");
            }
            return hb;
        }

        @ForceInline
        static long get(VarHandle ob, Object obb, int index) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            return SCOPED_MEMORY_ACCESS.getLongUnaligned(session(bb),
                    UNSAFE.getReference(bb, BYTE_BUFFER_HB),
                    ((long) index(bb, index)) + UNSAFE.getLong(bb, BUFFER_ADDRESS),
                    handle.be);
        }

        @ForceInline
        static void set(VarHandle ob, Object obb, int index, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            SCOPED_MEMORY_ACCESS.putLongUnaligned(session(bb),
                    UNSAFE.getReference(bb, BYTE_BUFFER_HB),
                    ((long) indexRO(bb, index)) + UNSAFE.getLong(bb, BUFFER_ADDRESS),
                    value,
                    handle.be);
        }

        @ForceInline
        static long getVolatile(VarHandle ob, Object obb, int index) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            return convEndian(handle.be,
                              SCOPED_MEMORY_ACCESS.getLongVolatile(session(bb),
                                      checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                                      address(bb, index(bb, index))));
        }

        @ForceInline
        static void setVolatile(VarHandle ob, Object obb, int index, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            SCOPED_MEMORY_ACCESS.putLongVolatile(session(bb),
                    checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                    address(bb, indexRO(bb, index)),
                    convEndian(handle.be, value));
        }

        @ForceInline
        static long getAcquire(VarHandle ob, Object obb, int index) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            return convEndian(handle.be,
                              SCOPED_MEMORY_ACCESS.getLongAcquire(session(bb),
                                      checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                                      address(bb, index(bb, index))));
        }

        @ForceInline
        static void setRelease(VarHandle ob, Object obb, int index, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            SCOPED_MEMORY_ACCESS.putLongRelease(session(bb),
                    checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                    address(bb, indexRO(bb, index)),
                    convEndian(handle.be, value));
        }

        @ForceInline
        static long getOpaque(VarHandle ob, Object obb, int index) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            return convEndian(handle.be,
                              SCOPED_MEMORY_ACCESS.getLongOpaque(session(bb),
                                      checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                                      address(bb, index(bb, index))));
        }

        @ForceInline
        static void setOpaque(VarHandle ob, Object obb, int index, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            SCOPED_MEMORY_ACCESS.putLongOpaque(session(bb),
                    checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                    address(bb, indexRO(bb, index)),
                    convEndian(handle.be, value));
        }

        @ForceInline
        static boolean compareAndSet(VarHandle ob, Object obb, int index, long expected, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            return SCOPED_MEMORY_ACCESS.compareAndSetLong(session(bb),
                    checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                    address(bb, indexRO(bb, index)),
                    convEndian(handle.be, expected), convEndian(handle.be, value));
        }

        @ForceInline
        static long compareAndExchange(VarHandle ob, Object obb, int index, long expected, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            return convEndian(handle.be,
                              SCOPED_MEMORY_ACCESS.compareAndExchangeLong(session(bb),
                                      checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                                      address(bb, indexRO(bb, index)),
                                      convEndian(handle.be, expected), convEndian(handle.be, value)));
        }

        @ForceInline
        static long compareAndExchangeAcquire(VarHandle ob, Object obb, int index, long expected, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            return convEndian(handle.be,
                              SCOPED_MEMORY_ACCESS.compareAndExchangeLongAcquire(session(bb),
                                      checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                                      address(bb, indexRO(bb, index)),
                                      convEndian(handle.be, expected), convEndian(handle.be, value)));
        }

        @ForceInline
        static long compareAndExchangeRelease(VarHandle ob, Object obb, int index, long expected, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            return convEndian(handle.be,
                              SCOPED_MEMORY_ACCESS.compareAndExchangeLongRelease(session(bb),
                                      checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                                      address(bb, indexRO(bb, index)),
                                      convEndian(handle.be, expected), convEndian(handle.be, value)));
        }

        @ForceInline
        static boolean weakCompareAndSetPlain(VarHandle ob, Object obb, int index, long expected, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            return SCOPED_MEMORY_ACCESS.weakCompareAndSetLongPlain(session(bb),
                    checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                    address(bb, indexRO(bb, index)),
                    convEndian(handle.be, expected), convEndian(handle.be, value));
        }

        @ForceInline
        static boolean weakCompareAndSet(VarHandle ob, Object obb, int index, long expected, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            return SCOPED_MEMORY_ACCESS.weakCompareAndSetLong(session(bb),
                    checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                    address(bb, indexRO(bb, index)),
                    convEndian(handle.be, expected), convEndian(handle.be, value));
        }

        @ForceInline
        static boolean weakCompareAndSetAcquire(VarHandle ob, Object obb, int index, long expected, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            return SCOPED_MEMORY_ACCESS.weakCompareAndSetLongAcquire(session(bb),
                    checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                    address(bb, indexRO(bb, index)),
                    convEndian(handle.be, expected), convEndian(handle.be, value));
        }

        @ForceInline
        static boolean weakCompareAndSetRelease(VarHandle ob, Object obb, int index, long expected, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            return SCOPED_MEMORY_ACCESS.weakCompareAndSetLongRelease(session(bb),
                    checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                    address(bb, indexRO(bb, index)),
                    convEndian(handle.be, expected), convEndian(handle.be, value));
        }

        @ForceInline
        static long getAndSet(VarHandle ob, Object obb, int index, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            return convEndian(handle.be,
                              SCOPED_MEMORY_ACCESS.getAndSetLong(session(bb),
                                      checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                                      address(bb, indexRO(bb, index)),
                                      convEndian(handle.be, value)));
        }

        @ForceInline
        static long getAndSetAcquire(VarHandle ob, Object obb, int index, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            return convEndian(handle.be,
                              SCOPED_MEMORY_ACCESS.getAndSetLongAcquire(session(bb),
                                      checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                                      address(bb, indexRO(bb, index)),
                                      convEndian(handle.be, value)));
        }

        @ForceInline
        static long getAndSetRelease(VarHandle ob, Object obb, int index, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            return convEndian(handle.be,
                              SCOPED_MEMORY_ACCESS.getAndSetLongRelease(session(bb),
                                      checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                                      address(bb, indexRO(bb, index)),
                                      convEndian(handle.be, value)));
        }

        @ForceInline
        static long getAndAdd(VarHandle ob, Object obb, int index, long delta) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            if (handle.be == BE) {
                return SCOPED_MEMORY_ACCESS.getAndAddLong(session(bb),
                        checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                        address(bb, indexRO(bb, index)),
                        delta);
            } else {
                return getAndAddConvEndianWithCAS(bb, index, delta);
            }
        }

        @ForceInline
        static long getAndAddAcquire(VarHandle ob, Object obb, int index, long delta) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            if (handle.be == BE) {
                return SCOPED_MEMORY_ACCESS.getAndAddLongAcquire(session(bb),
                        checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                        address(bb, indexRO(bb, index)),
                        delta);
            } else {
                return getAndAddConvEndianWithCAS(bb, index, delta);
            }
        }

        @ForceInline
        static long getAndAddRelease(VarHandle ob, Object obb, int index, long delta) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            if (handle.be == BE) {
                return SCOPED_MEMORY_ACCESS.getAndAddLongRelease(session(bb),
                        checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                        address(bb, indexRO(bb, index)),
                        delta);
            } else {
                return getAndAddConvEndianWithCAS(bb, index, delta);
            }
        }

        @ForceInline
        static long getAndAddConvEndianWithCAS(ByteBuffer bb, int index, long delta) {
            long nativeExpectedValue, expectedValue;
            Object base = checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB));
            long offset = address(bb, indexRO(bb, index));
            do {
                nativeExpectedValue = SCOPED_MEMORY_ACCESS.getLongVolatile(session(bb), base, offset);
                expectedValue = Long.reverseBytes(nativeExpectedValue);
            } while (!UNSAFE.weakCompareAndSetLong(base, offset,
                    nativeExpectedValue, Long.reverseBytes(expectedValue + delta)));
            return expectedValue;
        }

        @ForceInline
        static long getAndBitwiseOr(VarHandle ob, Object obb, int index, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            if (handle.be == BE) {
                return SCOPED_MEMORY_ACCESS.getAndBitwiseOrLong(session(bb),
                        checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                        address(bb, indexRO(bb, index)),
                        value);
            } else {
                return getAndBitwiseOrConvEndianWithCAS(bb, index, value);
            }
        }

        @ForceInline
        static long getAndBitwiseOrRelease(VarHandle ob, Object obb, int index, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            if (handle.be == BE) {
                return SCOPED_MEMORY_ACCESS.getAndBitwiseOrLongRelease(session(bb),
                        checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                        address(bb, indexRO(bb, index)),
                        value);
            } else {
                return getAndBitwiseOrConvEndianWithCAS(bb, index, value);
            }
        }

        @ForceInline
        static long getAndBitwiseOrAcquire(VarHandle ob, Object obb, int index, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            if (handle.be == BE) {
                return SCOPED_MEMORY_ACCESS.getAndBitwiseOrLongAcquire(session(bb),
                        checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                        address(bb, indexRO(bb, index)),
                        value);
            } else {
                return getAndBitwiseOrConvEndianWithCAS(bb, index, value);
            }
        }

        @ForceInline
        static long getAndBitwiseOrConvEndianWithCAS(ByteBuffer bb, int index, long value) {
            long nativeExpectedValue, expectedValue;
            Object base = checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB));
            long offset = address(bb, indexRO(bb, index));
            do {
                nativeExpectedValue = SCOPED_MEMORY_ACCESS.getLongVolatile(session(bb), base, offset);
                expectedValue = Long.reverseBytes(nativeExpectedValue);
            } while (!UNSAFE.weakCompareAndSetLong(base, offset,
                    nativeExpectedValue, Long.reverseBytes(expectedValue | value)));
            return expectedValue;
        }

        @ForceInline
        static long getAndBitwiseAnd(VarHandle ob, Object obb, int index, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            if (handle.be == BE) {
                return SCOPED_MEMORY_ACCESS.getAndBitwiseAndLong(session(bb),
                        checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                        address(bb, indexRO(bb, index)),
                        value);
            } else {
                return getAndBitwiseAndConvEndianWithCAS(bb, index, value);
            }
        }

        @ForceInline
        static long getAndBitwiseAndRelease(VarHandle ob, Object obb, int index, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            if (handle.be == BE) {
                return SCOPED_MEMORY_ACCESS.getAndBitwiseAndLongRelease(session(bb),
                        checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                        address(bb, indexRO(bb, index)),
                        value);
            } else {
                return getAndBitwiseAndConvEndianWithCAS(bb, index, value);
            }
        }

        @ForceInline
        static long getAndBitwiseAndAcquire(VarHandle ob, Object obb, int index, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            if (handle.be == BE) {
                return SCOPED_MEMORY_ACCESS.getAndBitwiseAndLongAcquire(session(bb),
                        checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                        address(bb, indexRO(bb, index)),
                        value);
            } else {
                return getAndBitwiseAndConvEndianWithCAS(bb, index, value);
            }
        }

        @ForceInline
        static long getAndBitwiseAndConvEndianWithCAS(ByteBuffer bb, int index, long value) {
            long nativeExpectedValue, expectedValue;
            Object base = checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB));
            long offset = address(bb, indexRO(bb, index));
            do {
                nativeExpectedValue = SCOPED_MEMORY_ACCESS.getLongVolatile(session(bb), base, offset);
                expectedValue = Long.reverseBytes(nativeExpectedValue);
            } while (!UNSAFE.weakCompareAndSetLong(base, offset,
                    nativeExpectedValue, Long.reverseBytes(expectedValue & value)));
            return expectedValue;
        }


        @ForceInline
        static long getAndBitwiseXor(VarHandle ob, Object obb, int index, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            if (handle.be == BE) {
                return SCOPED_MEMORY_ACCESS.getAndBitwiseXorLong(session(bb),
                        checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                        address(bb, indexRO(bb, index)),
                        value);
            } else {
                return getAndBitwiseXorConvEndianWithCAS(bb, index, value);
            }
        }

        @ForceInline
        static long getAndBitwiseXorRelease(VarHandle ob, Object obb, int index, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            if (handle.be == BE) {
                return SCOPED_MEMORY_ACCESS.getAndBitwiseXorLongRelease(session(bb),
                        checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                        address(bb, indexRO(bb, index)),
                        value);
            } else {
                return getAndBitwiseXorConvEndianWithCAS(bb, index, value);
            }
        }

        @ForceInline
        static long getAndBitwiseXorAcquire(VarHandle ob, Object obb, int index, long value) {
            ByteBufferHandle handle = (ByteBufferHandle)ob;
            ByteBuffer bb = (ByteBuffer) Objects.requireNonNull(obb);
            if (handle.be == BE) {
                return SCOPED_MEMORY_ACCESS.getAndBitwiseXorLongAcquire(session(bb),
                        checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB)),
                        address(bb, indexRO(bb, index)),
                        value);
            } else {
                return getAndBitwiseXorConvEndianWithCAS(bb, index, value);
            }
        }

        @ForceInline
        static long getAndBitwiseXorConvEndianWithCAS(ByteBuffer bb, int index, long value) {
            long nativeExpectedValue, expectedValue;
            Object base = checkNullHeapBase(UNSAFE.getReference(bb, BYTE_BUFFER_HB));
            long offset = address(bb, indexRO(bb, index));
            do {
                nativeExpectedValue = SCOPED_MEMORY_ACCESS.getLongVolatile(session(bb), base, offset);
                expectedValue = Long.reverseBytes(nativeExpectedValue);
            } while (!UNSAFE.weakCompareAndSetLong(base, offset,
                    nativeExpectedValue, Long.reverseBytes(expectedValue ^ value)));
            return expectedValue;
        }

        static final VarForm FORM = new VarForm(ByteBufferHandle.class, ByteBuffer.class, long.class, int.class);
    }
}
