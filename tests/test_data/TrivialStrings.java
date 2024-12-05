package samples.javacore.strings.trivial;

public class TrivialStrings {
    public static void main(String[] args) {
        char[] textArray = {'J', 'a', 'v', 'a', ' ', 'i', 's', ' ', 'f', 'l', 'e', 'x', 'i', 'b', 'l', 'e'};
        String text = new String(textArray);
        char[] searchArray = {'f', 'l', 'e', 'x', 'i', 'b', 'l', 'e'};
        String search = new String(searchArray);

        int index = text.indexOf(search);
        System.out.println(index);
    }
}
