package samples.net.networkinterfaces;

import java.net.NetworkInterface;
import java.util.Collections;

public class NetworkInterfacesExample {
    public static void main(String[] args) throws Exception {
        int count = 0;
        int addresses = 0;
        boolean validNames = true;
        for (NetworkInterface networkInterface : Collections.list(NetworkInterface.getNetworkInterfaces())) {
            count++;
            addresses += Collections.list(networkInterface.getInetAddresses()).size();
            validNames &= networkInterface.getName() != null && !networkInterface.getName().isEmpty();
            validNames &= networkInterface.getIndex() > 0;
        }
        System.out.println("count=" + count);
        System.out.println("addresses=" + addresses);
        System.out.println("valid names=" + validNames);
    }
}
