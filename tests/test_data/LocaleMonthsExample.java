package samples.locale.localemonthsexample;

import java.text.DateFormatSymbols;
import java.util.Arrays;
import java.util.Locale;

public class LocaleMonthsExample {
    public static void main(String[] args) {
        Locale ukrainian = Locale.of("uk", "UA");
        String[] months = new DateFormatSymbols(ukrainian).getMonths();
        System.out.println(Arrays.toString(months));
    }
}
