package samples.system.setin;

import java.io.ByteArrayInputStream;

public class SystemSetInExample {
    public static void main(String[] args) throws Exception {
        System.setIn(new ByteArrayInputStream(new byte[] { 65, 66 }));
        System.out.println(System.in.read());
        System.out.println(System.in.read());
    }
}
