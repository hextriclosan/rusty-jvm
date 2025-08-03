package samples.regex.regexexample;

import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class RegexExample {
    public static void main(String[] args) {
        basic();
        advanced();
    }

    private static void basic() {
        String input = "Contact us at support@example.com or sales@example.org.";

        // Pattern to match email addresses
        String regex = "(\\w+)@([\\w.]+)";

        Pattern pattern = Pattern.compile(regex);
        Matcher matcher = pattern.matcher(input);

        // Find and print all email addresses
        while (matcher.find()) {
            System.out.println("Full match: " + matcher.group(0));
            System.out.println("Username: " + matcher.group(1));
            System.out.println("Domain: " + matcher.group(2));
            System.out.println("---");
        }

        // Replace all emails with [email hidden]
        String censored = matcher.replaceAll("[email hidden]");
        System.out.println("Censored text: " + censored);
    }

    private static void advanced() {
        String input = """
                User: Alice (email: alice@example.com)
                User: Bob (email: bob@EXAMPLE.org)
                Invalid: eve@not-an-email
                User: Carol (email: carol@sub.example.co.uk)
                """;

        // Advanced regex:
        // - Named groups for user and domain
        // - Case-insensitive match
        // - Ensure "User: " prefix with lookbehind
        String regex = "(?m)^User: (?<user>\\w+) \\(email: (?<email>(?<username>\\w+)@(?<domain>[\\w.]+))\\)$";

        Pattern pattern = Pattern.compile(regex, Pattern.CASE_INSENSITIVE);
        Matcher matcher = pattern.matcher(input);

        while (matcher.find()) {
            System.out.println("User: " + matcher.group("user"));
            System.out.println("  Email: " + matcher.group("email"));
            System.out.println("  Username: " + matcher.group("username"));
            System.out.println("  Domain: " + matcher.group("domain"));

            // Example of using a positive lookahead (not captured here, but illustrated)
            if (matcher.group("domain").matches("(?i).*example\\.com$")) {
                System.out.println("  > Corporate email detected!");
            }
            System.out.println();
        }
    }
}
