public class Array3D {
    public static void main(String[] args) {
        int[][][] array = new int[][][]{
                {{10, 20}, {30, 40}, {50, 60}},
                {{70, 80}, {90, 100}, {110, 120}}
        };

        int result = sum(array);
    }

    private static int sum(int[][][] array3d) {
        int sum = 0;
        for (var array2d : array3d) {
            for (var array1d: array2d) {
                for (var value : array1d) {
                    sum += value;
                }
            }
        }

        return sum;
    }

}
