package samples.locale.showlocale;

import java.time.LocalDate;
import java.time.format.DateTimeFormatter;
import java.time.format.FormatStyle;
import java.util.List;
import java.util.Locale;
import java.text.NumberFormat;
import java.text.DateFormat;
import java.util.Date;

public class ShowLocale {
    public static void main(String[] args) {
        List<Locale> locales = List.of(
                //Locale.getDefault(), // https://github.com/hextriclosan/rusty-jvm/issues/489
                Locale.US,
                Locale.FRANCE,
                Locale.JAPAN,
                Locale.forLanguageTag("uk-UA"),
                new Locale.Builder().setLanguage("es").setRegion("MX").build()
        );

        for (Locale loc : locales) {
            System.out.println("--- Locale: " + loc + " ---");
            printLocaleInfo(loc);
            printFormattingExamples(loc);
            System.out.println();
        }
    }

    private static void printLocaleInfo(Locale locale) {
        System.out.println("Language:           '" + locale.getLanguage() + "'");
        System.out.println("Country:            '" + locale.getCountry() + "'");
        System.out.println("Variant:            '" + locale.getVariant() + "'");
        System.out.println("Script:             '" + locale.getScript() + "'");
        System.out.println("Display Language:   '" + locale.getDisplayLanguage() + "'");
        System.out.println("Display Country:    '" + locale.getDisplayCountry() + "'");
        System.out.println("Display Name:       '" + locale.getDisplayName() + "'");
        System.out.println("Display Variant:    '" + locale.getDisplayVariant() + "'");
        System.out.println("Display Script:     '" + locale.getDisplayScript() + "'");
        System.out.println("ISO3 Language:      '" + locale.getISO3Language() + "'");
        System.out.println("ISO3 Country:       '" + locale.getISO3Country() + "'");
    }

    private static void printFormattingExamples(Locale locale) {
        NumberFormat numberFormat = NumberFormat.getNumberInstance(locale);
        NumberFormat currencyFormat = NumberFormat.getCurrencyInstance(locale);
        DateFormat dateFormat = DateFormat.getDateInstance(DateFormat.LONG, locale);
        Date now = new Date(91, 7, 24); // August 24, 1991

        double exampleNumber = 1234567.89;

        System.out.println("Number Format:   " + numberFormat.format(exampleNumber));
        System.out.println("Currency Format: " + currencyFormat.format(exampleNumber));
        System.out.println("Date Format:     " + dateFormat.format(now));
    }

    //fixme: VM execution failed: Native Call Error: Native method java/lang/invoke/VarHandle:compareAndSet not found
//     private static void printFormattingExamples(Locale locale) {
//         NumberFormat numberFormat = NumberFormat.getNumberInstance(locale);
//         NumberFormat currencyFormat = NumberFormat.getCurrencyInstance(locale);
//
//         LocalDate now = LocalDate.of(1991, Month.AUGUST, 24);
//         DateTimeFormatter dateFormatter = DateTimeFormatter
//                 .ofLocalizedDate(FormatStyle.FULL)
//                 .withLocale(locale);
//
//         double exampleNumber = 1_234_567.89;
//
//         System.out.println("Number Format:   " + numberFormat.format(exampleNumber));
//         System.out.println("Currency Format: " + currencyFormat.format(exampleNumber));
//         System.out.println("Date Format:     " + now.format(dateFormatter));
//     }
}
