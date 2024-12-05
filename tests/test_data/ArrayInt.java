package samples.arrays.array.ints;

public class ArrayInt {

    public static void main(String[] args) {
        // 1. Initialize two arrays with different sizes
        int[] numbers1 = {5, 3, 8, 12, 7};
        int[] numbers2 = {14, 22, 16, 9};

        // 2. Combine two arrays dynamically
        int[] combinedArray = combineArrays(numbers1, numbers2);

        // 3. Modify the combined array by squaring even numbers
        squareEvenNumbers(combinedArray);

        // 4. Double the size of the combined array dynamically
        combinedArray = resizeArray(combinedArray, combinedArray.length * 2);

        // 5. Fill the new half of the array with values
        for (int i = combinedArray.length / 2; i < combinedArray.length; i++) {
            combinedArray[i] = i * 2; // Fill new slots with multiples of 2
        }

        // 6. Find the second largest element in the combined array
        int secondLargest = findSecondLargest(combinedArray);

        // 7. Reverse the combined array
        reverseArray(combinedArray);


        // 8. Shift array elements to the left by 11 positions
        shiftLeft(combinedArray, 11);

        int result = combinedArray[0] + secondLargest;
        System.out.println(result);
    }

    // Method to combine two arrays into one dynamically
    private static int[] combineArrays(int[] array1, int[] array2) {
        int newLength = array1.length + array2.length;
        int[] result = new int[newLength];

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
    private static void squareEvenNumbers(int[] array) {
        for (int i = 0; i < array.length; i++) {
            if (array[i] % 2 == 0) {
                array[i] = array[i] * array[i];
            }
        }
    }

    // Method to resize the array dynamically
    private static int[] resizeArray(int[] array, int newSize) {
        int[] newArray = new int[newSize];
        for (int i = 0; i < array.length; i++) {
            newArray[i] = array[i];
        }
        return newArray;
    }

    // Method to find the second largest element in the array
    private static int findSecondLargest(int[] array) {
        int largest = Integer.MIN_VALUE;
        int secondLargest = Integer.MIN_VALUE;

        for (int num : array) {
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
    private static void reverseArray(int[] array) {
        int start = 0;
        int end = array.length - 1;
        while (start < end) {
            int temp = array[start];
            array[start] = array[end];
            array[end] = temp;
            start++;
            end--;
        }
    }

    // Method to shift the array elements to the left by a given number of positions
    private static void shiftLeft(int[] array, int positions) {
        int length = array.length;
        int[] temp = new int[positions];

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
