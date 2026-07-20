package samples.exceptions.proper.handling.of.fillinstacktrace;

import java.util.Arrays;

public class FillInStackTraceExplicit {
    public static void main(String[] args) {
        Exception e = new Exception("boom");
        refill(e);
        System.out.println(Arrays.toString(e.getStackTrace()));
    }

    static void refill(Exception e) {
        e.fillInStackTrace();
    }
}
