package samples.shutdown.exitwithcodedemo;

import java.io.File;
import java.io.FileInputStream;

public class ExitWithCodeDemo {
    public static void main(String[] args) throws Exception {
        int code = Integer.parseInt(args[0]);
        System.out.print("Exiting with code: " + code);
        System.exit(code);
    }
}
