package samples.javacore.strings.cpool.trivial;

public class TrivialStringsCPool {
    public static void main(String[] args) {
        String text = "Java is flexible";
        String search = new String("flexible");

        int index = text.indexOf(search);
        System.out.println(index);
    }
}
