package samples.nativecall.system;

import java.util.Arrays;

public class NativeCallSystemArrayCopy {
    private static final long L1 =  42_949_672_980L;  // h=10,  l=20
    private static final long L2 = 128_849_018_920L;  // h=30,  l=40
    private static final long L3 = 214_748_364_860L;  // h=50,  l=60
    private static final long L4 = 300_647_710_800L;  // h=70,  l=80
    private static final long L5 = 386_547_056_740L;  // h=90,  l=100
    private static final long L6 = 472_446_402_680L;  // h=110, l=120

    public static void main(String[] args) {
        System.out.println("=== POSITIVE CASES ===");
        positive_intArrayCopy();
        positive_longArrayCopy();
        positive_overlappingInt();
        positive_overlappingLong();
        positive_upcasting();
        positive_runtimeCompatible();

        System.out.println("=== NEGATIVE CASES ===");
        negative_nullSrc();
        negative_nullDest();
        negative_srcNotArray();
        negative_destNotArray();
        negative_srcPrimitive_destReference();
        negative_srcReference_destPrimitive();
        negative_differentPrimitiveTypes();
        negative_incompatibleReferences();
        negative_srcPosNegative();
        negative_destPosNegative();
        negative_lengthNegative();
        negative_srcPosPlusLengthTooBig();
        negative_destPosPlusLengthTooBig();
    }


    /* ============================
       POSITIVE CASES
       ============================ */

    private static void positive_intArrayCopy() {
        int[] src = {10, 20, 30, 40, 50};
        int[] dest = {1, 2, 3, 4, 5};
        System.arraycopy(src, 1, dest, 2, 3);
        System.out.println("positive_intArrayCopy: " + Arrays.toString(dest));
    }

    private static void positive_longArrayCopy() {
        long[] src  = {L1, L2, L3, L4, L5, L6};
        long[] dest = {1, 2, 3, 4, 5, 6};

        System.arraycopy(src, 1, dest, 2, 3);
        System.out.println("positive_longArrayCopy: " + Arrays.toString(dest));
    }

    private static void positive_overlappingInt() {
        int[] arr = {10, 20, 30, 40, 50, 60};
        System.arraycopy(arr, 0, arr, 1, 5);
        System.out.println("positive_overlappingInt: " + Arrays.toString(arr));
    }

    private static void positive_overlappingLong() {
        long[] arr = {L1, L2, L3, L4, L5, L6};
        System.arraycopy(arr, 0, arr, 1, 5);
        System.out.println("positive_overlappingLong: " + Arrays.toString(arr));
    }

    private static void positive_upcasting() {
        Integer[] src = {10, 20, 30};
        Number[] dest = new Number[3];
        System.arraycopy(src, 0, dest, 0, 3);
        System.out.println("positive_upcasting: " + Arrays.toString(dest));
    }

    private static void positive_runtimeCompatible() {
        System.out.print("positive_runtimeCompatible: ");
        Number[] src = { 3.14, 2.71, 1.41 }; // actually Doubles
        Double[] dest = new Double[3];

        System.arraycopy(src, 0, dest, 0, 3);
        System.out.println(Arrays.toString(dest));
    }

    /* ============================
       NEGATIVE CASES
       ============================ */

    private static void negative_nullSrc() {
        System.out.print("negative_nullSrc: ");
        try {
            long[] dest = {L1, L2, L3};
            System.arraycopy(null, 0, dest, 0, 1);
        } catch (Exception e) {
            System.out.println(e);
        }
    }

    private static void negative_nullDest() {
        System.out.print("negative_nullDest: ");
        try {
            long[] src = {L1, L2, L3};
            System.arraycopy(src, 0, null, 0, 1);
        } catch (Exception e) {
            System.out.println(e);
        }
    }

    private static void negative_srcNotArray() {
        System.out.print("negative_srcNotArray: ");
        try {
            Object src = "not array";
            long[] dest = {L1, L2, L3};
            System.arraycopy(src, 0, dest, 0, 1);
        } catch (Exception e) {
            System.out.println(e);
        }
    }

    private static void negative_destNotArray() {
        System.out.print("negative_destNotArray: ");
        try {
            long[] src = {L1, L2, L3};
            Object dest = 12345L;
            System.arraycopy(src, 0, dest, 0, 1);
        } catch (Exception e) {
            System.out.println(e);
        }
    }

    private static void negative_srcPrimitive_destReference() {
        System.out.print("negative_srcPrimitive_destReference: ");
        try {
            long[] src = {L1, L2, L3};
            Object[] dest = new Object[3];
            System.arraycopy(src, 0, dest, 0, 1);
        } catch (Exception e) {
            System.out.println(e);
        }
    }

    private static void negative_srcReference_destPrimitive() {
        System.out.print("negative_srcReference_destPrimitive: ");
        try {
            Object[] src = {L1, L2, L3};
            long[] dest = new long[3];
            System.arraycopy(src, 0, dest, 0, 1);
        } catch (Exception e) {
            System.out.println(e);
        }
    }

    private static void negative_differentPrimitiveTypes() {
        System.out.print("negative_differentPrimitiveTypes: ");
        try {
            long[] src = {L1, L2, L3};
            int[] dest = {1, 2, 3};
            System.arraycopy(src, 0, dest, 0, 1);
        } catch (Exception e) {
            System.out.println(e);
        }
    }

    private static void negative_incompatibleReferences() {
        System.out.print("negative_incompatibleReferences: ");
        Double[] dest = {3.14, 2.71, 1.41};
        try {
            Number[] src = {3.0, 2.0, 1};
            System.arraycopy(src, 0, dest, 0, 3);
        } catch (Exception e) {
            System.out.print(e);
        }
        System.out.println(" destination after copying: " + Arrays.toString(dest));
    }

    private static void negative_srcPosNegative() {
        System.out.print("negative_srcPosNegative: ");
        try {
            long[] src = {L1, L2, L3};
            long[] dest = {L4, L5, L6};
            System.arraycopy(src, -1, dest, 0, 1);
        } catch (Exception e) {
            System.out.println(e);
        }
    }

    private static void negative_destPosNegative() {
        System.out.print("negative_destPosNegative: ");
        try {
            long[] src = {L1, L2, L3};
            long[] dest = {L4, L5, L6};
            System.arraycopy(src, 0, dest, -1, 1);
        } catch (Exception e) {
            System.out.println(e);
        }
    }

    private static void negative_lengthNegative() {
        System.out.print("negative_lengthNegative: ");
        try {
            long[] src = {L1, L2, L3};
            long[] dest = {L4, L5, L6};
            System.arraycopy(src, 0, dest, 0, -1);
        } catch (Exception e) {
            System.out.println(e);
        }
    }

    private static void negative_srcPosPlusLengthTooBig() {
        System.out.print("negative_srcPosPlusLengthTooBig: ");
        try {
            long[] src = {L1, L2, L3};
            long[] dest = {L4, L5, L6};
            System.arraycopy(src, 1, dest, 0, 3); // 1+3 > 3
        } catch (Exception e) {
            System.out.println(e);
        }
    }

    private static void negative_destPosPlusLengthTooBig() {
        System.out.print("negative_destPosPlusLengthTooBig: ");
        try {
            long[] src = {L1, L2, L3};
            long[] dest = {L4, L5, L6};
            System.arraycopy(src, 0, dest, 2, 2); // 2+2 > 3
        } catch (Exception e) {
            System.out.println(e);
        }
    }
}
