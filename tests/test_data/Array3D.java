package samples.arrays.array3d;

public class Array3D {
    public static void main(String[] args) {
        int[][][] initialization = new int[][][]{
                {{10, 20}, {30, 40}, {50, 60}},
                {{70, 80}, {90, 100}, {110, 120}}
        };

        int[][][] dimensionExpression = new int[2][1][4];
        dimensionExpression[0][0][0] = 10;
        dimensionExpression[0][0][1] = 20;
        dimensionExpression[0][0][2] = 30;
        dimensionExpression[0][0][3] = 40;
        dimensionExpression[1][0][0] = 50;
        dimensionExpression[1][0][1] = 60;
        dimensionExpression[1][0][2] = 70;
        dimensionExpression[1][0][3] = 80;

        System.out.println(sum(initialization));
        System.out.println(sum(dimensionExpression));
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
