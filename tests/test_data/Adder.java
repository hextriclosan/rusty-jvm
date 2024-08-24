//javac -g -parameters Adder.java

public class Adder {

    public static void main(String[] args) {
         int result = add(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    }

    private static int add(int v1, int v2, int v3, int v4, int v5, int v6, int v7, int v8, int v9, int v10) {
        return v1 + v2 + v3 + v4 + v5 + v6 + v7 + v8 + v9 + v10;
    }

}
