
public class SubNew {

    public SubNew() {
    }

    public static void main(String[] args) {
        int first = 11;
        int second = 1000;
        SubNew subNew = new SubNew();
        int result = subNew.sub(first, second);
    }

    private int sub(int first, int second) {
        return first - second;
    }

}
