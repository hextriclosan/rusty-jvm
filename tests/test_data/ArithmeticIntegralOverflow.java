package samples.arithmetics.overflow;

public class ArithmeticIntegralOverflow {
    public static void main(String[] args) {
        checkOverflow(Byte.MIN_VALUE, Byte.MAX_VALUE, (byte) 1, (byte) 2, (byte) -1, new ByteOperations());
        checkOverflow(Short.MIN_VALUE, Short.MAX_VALUE, (short) 1, (short) 2, (short) -1, new ShortOperations());
        checkOverflow(Integer.MIN_VALUE, Integer.MAX_VALUE, 1, 2, -1, new IntOperations());
        checkOverflow(Long.MIN_VALUE, Long.MAX_VALUE, 1L, 2L, -1L, new LongOperations());
    }

    private static <T> void checkOverflow(T minValue, T maxValue, T one, T two, T minusOne, Operations<T> operations) {
        System.out.print(minValue.getClass());
        System.out.println(":");
        System.out.print("add overflow: ");
        System.out.println(operations.addOverflow(maxValue, one));
        System.out.print("mul overflow: ");
        System.out.println(operations.mulOverflow(maxValue, two));
        System.out.print("div overflow: ");
        System.out.println(operations.divOverflow(minValue, minusOne));
        System.out.print("neg overflow: ");
        System.out.println(operations.negOverflow(minValue));
        System.out.print("rem overflow: ");
        System.out.println(operations.remOverflow(minValue, minusOne));
        System.out.print("shl overflow: ");
        System.out.println(operations.shlOverflow(maxValue, 1));
        System.out.print("shr overflow: ");
        System.out.println(operations.shrOverflow(minValue, 1));
        System.out.print("ushl overflow: ");
        System.out.println(operations.uShlOverflow(minValue, 1));
        System.out.print("sub overflow: ");
        System.out.println(operations.subOverflow(minValue, one));
        System.out.println();
    }

    private interface Operations<T> {
        T addOverflow(T value1, T value2);
        T mulOverflow(T value1, T value2);
        T divOverflow(T value1, T value2);
        T negOverflow(T value);
        T remOverflow(T value1, T value2);
        T shlOverflow(T value, int bits);
        T shrOverflow(T value, int bits);
        T uShlOverflow(T value, int bits);
        T subOverflow(T value1, T value2);
    }

    private static class ByteOperations implements Operations<Byte> {
        @Override
        public Byte addOverflow(Byte value1, Byte value2) {
            return (byte) (value1 + value2);
        }

        @Override
        public Byte mulOverflow(Byte value1, Byte value2) {
            return (byte) (value1 * value2);
        }

        @Override
        public Byte divOverflow(Byte value1, Byte value2) {
            return (byte) (value1 / value2);
        }

        @Override
        public Byte negOverflow(Byte value) {
            return (byte) (-value);
        }

        @Override
        public Byte remOverflow(Byte value1, Byte value2) {
            return (byte) (value1 % value2);
        }

        @Override
        public Byte shlOverflow(Byte value, int bits) {
            return (byte) (value << bits);
        }

        @Override
        public Byte shrOverflow(Byte value, int bits) {
            return (byte) (value >> bits);
        }

        @Override
        public Byte uShlOverflow(Byte value, int bits) {
            return (byte) (value >>> bits);
        }

        @Override
        public Byte subOverflow(Byte value1, Byte value2) {
            return (byte) (value1 - value2);
        }
    }

    private static class ShortOperations implements Operations<Short> {
        @Override
        public Short addOverflow(Short value1, Short value2) {
            return (short) (value1 + value2);
        }

        @Override
        public Short mulOverflow(Short value1, Short value2) {
            return (short) (value1 * value2);
        }

        @Override
        public Short divOverflow(Short value1, Short value2) {
            return (short) (value1 / value2);
        }

        @Override
        public Short negOverflow(Short value) {
            return (short) (-value);
        }

        @Override
        public Short remOverflow(Short value1, Short value2) {
            return (short) (value1 % value2);
        }

        @Override
        public Short shlOverflow(Short value, int bits) {
            return (short) (value << bits);
        }

        @Override
        public Short shrOverflow(Short value, int bits) {
            return (short) (value >> bits);
        }

        @Override
        public Short uShlOverflow(Short value, int bits) {
            return (short) (value >>> bits);
        }

        @Override
        public Short subOverflow(Short value1, Short value2) {
            return (short) (value1 - value2);
        }
    }

    private static class IntOperations implements Operations<Integer> {
        @Override
        public Integer addOverflow(Integer value1, Integer value2) {
            return value1 + value2;
        }

        @Override
        public Integer mulOverflow(Integer value1, Integer value2) {
            return value1 * value2;
        }

        @Override
        public Integer divOverflow(Integer value1, Integer value2) {
            return value1 / value2;
        }

        @Override
        public Integer negOverflow(Integer value) {
            return -value;
        }

        @Override
        public Integer remOverflow(Integer value1, Integer value2) {
            return value1 % value2;
        }

        @Override
        public Integer shlOverflow(Integer value, int bits) {
            return value << bits;
        }

        @Override
        public Integer shrOverflow(Integer value, int bits) {
            return value >> bits;
        }

        @Override
        public Integer uShlOverflow(Integer value, int bits) {
            return value >>> bits;
        }

        @Override
        public Integer subOverflow(Integer value1, Integer value2) {
            return value1 - value2;
        }
    }

    private static class LongOperations implements Operations<Long> {
        @Override
        public Long addOverflow(Long value1, Long value2) {
            return value1 + value2;
        }

        @Override
        public Long mulOverflow(Long value1, Long value2) {
            return value1 * value2;
        }

        @Override
        public Long divOverflow(Long value1, Long value2) {
            return value1 / value2;
        }

        @Override
        public Long negOverflow(Long value) {
            return -value;
        }

        @Override
        public Long remOverflow(Long value1, Long value2) {
            return value1 % value2;
        }

        @Override
        public Long shlOverflow(Long value, int bits) {
            return value << bits;
        }

        @Override
        public Long shrOverflow(Long value, int bits) {
            return value >> bits;
        }

        @Override
        public Long uShlOverflow(Long value, int bits) {
            return value >>> bits;
        }

        @Override
        public Long subOverflow(Long value1, Long value2) {
            return value1 - value2;
        }
    }
}
