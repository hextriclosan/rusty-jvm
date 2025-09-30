package samples.network.trivial;

import java.net.NetworkInterface;
import java.util.Enumeration;

public class NetworkInterfaceGetAllExample {
    public static void main(String[] args) {
        try {
            Enumeration<NetworkInterface> interfaces = NetworkInterface.getNetworkInterfaces();
            
            if (interfaces == null) {
                System.out.println("0");
            } else {
                int count = 0;
                while (interfaces.hasMoreElements()) {
                    interfaces.nextElement();
                    count++;
                }
                System.out.println(count);
            }
        } catch (Exception e) {
            e.printStackTrace();
            System.out.println("-1");
        }
    }
}
