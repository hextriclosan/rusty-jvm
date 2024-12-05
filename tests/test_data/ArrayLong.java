package samples.arrays.array.longs;

public class ArrayLong {

    public static void main(String[] args) {
        long result = calculate();
        System.out.println(result);
    }

    private static long calculate() {
        // 1. Initialize two arrays with different sizes
        long[] numbers1 = {
                42_949_672_980l/*h=10,l=20*/,
                128_849_018_920l/*h=30,l=40*/,
                214_748_364_860l/*h=50,l=60*/,
                300_647_710_800l/*h=70,l=80*/
        };
        long[] numbers2 = {
                386_547_056_740l/*h=90,l=100*/,
                472_446_402_680l/*h=110,l=120*/,
                558_345_748_620l/*h=130,l=140*/,
                644_245_094_560l/*h=150,l=160*/
        };

        // 2. Combine two arrays dynamically
        long[] combinedArray = combineArrays(numbers1, numbers2);

        // 3. Modify the combined array by squaring even numbers
        squareEvenNumbers(combinedArray);

        // 4. Double the size of the combined array dynamically
        combinedArray = resizeArray(combinedArray, combinedArray.length * 2);

        // 5. Fill the new half of the array with values
        for (int i = combinedArray.length / 2; i < combinedArray.length; i++) {
            combinedArray[i] = i * 2; // Fill new slots with multiples of 2
        }

        // 6. Find the second largest element in the combined array
        long secondLargest = findSecondLargest(combinedArray);

        // 7. Reverse the combined array
        reverseArray(combinedArray);


        // 8. Shift array elements to the left by 11 positions
        shiftLeft(combinedArray, 11);

        return combinedArray[0] + secondLargest;
    }

    // Method to combine two arrays into one dynamically
    private static long[] combineArrays(long[] array1, long[] array2) {
        int newLength = array1.length + array2.length;
        long[] result = new long[newLength];

        // Copy elements from the first array
        for (int i = 0; i < array1.length; i++) {
            result[i] = array1[i];
        }

        // Copy elements from the second array
        for (int i = 0; i < array2.length; i++) {
            result[array1.length + i] = array2[i];
        }

        return result;
    }

    // Method to square even numbers in the array
    private static void squareEvenNumbers(long[] array) {
        for (int i = 0; i < array.length; i++) {
            if (array[i] % 2 == 0) {
                array[i] = array[i] * array[i];
            }
        }
    }

    // Method to resize the array dynamically
    private static long[] resizeArray(long[] array, int newSize) {
        long[] newArray = new long[newSize];
        for (int i = 0; i < array.length; i++) {
            newArray[i] = array[i];
        }
        return newArray;
    }

    // Method to find the second largest element in the array
    private static long findSecondLargest(long[] array) {
        long largest = Long.MIN_VALUE;
        long secondLargest = Long.MIN_VALUE;

        for (long num : array) {
            if (num > largest) {
                secondLargest = largest;
                largest = num;
            } else if (num > secondLargest && num != largest) {
                secondLargest = num;
            }
        }
        return secondLargest;
    }

    // Method to reverse the array in place
    private static void reverseArray(long[] array) {
        int start = 0;
        int end = array.length - 1;
        while (start < end) {
            long temp = array[start];
            array[start] = array[end];
            array[end] = temp;
            start++;
            end--;
        }
    }

    // Method to shift the array elements to the left by a given number of positions
    private static void shiftLeft(long[] array, int positions) {
        int length = array.length;
        long[] temp = new long[positions];

        // Store the first 'positions' elements in a temporary array
        for (int i = 0; i < positions; i++) {
            temp[i] = array[i];
        }

        // Shift the remaining elements to the left
        for (int i = positions; i < length; i++) {
            array[i - positions] = array[i];
        }

        // Place the stored elements at the end
        for (int i = 0; i < positions; i++) {
            array[length - positions + i] = temp[i];
        }
    }
}
