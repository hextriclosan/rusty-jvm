package samples.properties;

import java.nio.charset.Charset;

public class EncodingPropertiesDemo {
    public static void main(String[] args) {
        for (String property : new String[] {"native.encoding", "stdin.encoding", "stdout.encoding", "stderr.encoding"}) {
            String value = System.getProperty(property);
            System.out.println(property + "=" + value);
            if (value != null && !Charset.isSupported(value)) {
                throw new AssertionError(property + "=" + value);
            }
        }
        System.out.println("encoding-properties-ok");
    }
}
