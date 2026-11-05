package samples.system.localeproperties;

import java.util.Locale;

public class LocaleProperties {
    public static void main(String[] args) {
        for (String part : new String[] {"language", "script", "country", "variant"}) {
            for (String suffix : new String[] {"", ".display", ".format"}) {
                String key = "user." + part + suffix;
                System.out.println(key + "=" + System.getProperty(key));
            }
        }
        System.out.println("default=" + Locale.getDefault().toLanguageTag());
        System.out.println("display=" + Locale.getDefault(Locale.Category.DISPLAY).toLanguageTag());
        System.out.println("format=" + Locale.getDefault(Locale.Category.FORMAT).toLanguageTag());
    }
}
