package samples.net.networkinterfaces;

import java.net.InetAddress;
import java.net.NetworkInterface;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;

public class NetworkInterfacesExample {
    public static void main(String[] args) throws Exception {
        List<NetworkInterface> interfaces = Collections.list(NetworkInterface.getNetworkInterfaces());
        System.out.println("interfaces: " + interfaces.size());
        for (NetworkInterface networkInterface : interfaces) {
            describe(networkInterface);
        }
        System.out.println("stream: " + NetworkInterface.networkInterfaces().count());

        for (NetworkInterface networkInterface : interfaces) {
            String name = networkInterface.getName();
            System.out.println("byName " + name + ": " + NetworkInterface.getByName(name)
                    + " equal=" + networkInterface.equals(NetworkInterface.getByName(name)));
            System.out.println("byIndex " + networkInterface.getIndex() + ": "
                    + NetworkInterface.getByIndex(networkInterface.getIndex()));
            for (InetAddress address : Collections.list(networkInterface.getInetAddresses())) {
                System.out.println("byInetAddress " + address + ": " + NetworkInterface.getByInetAddress(address));
            }
        }

        System.out.println("missing name: " + NetworkInterface.getByName("missing0"));
        System.out.println("missing virtual name: " + NetworkInterface.getByName("missing0:1"));
        System.out.println("index 0: " + NetworkInterface.getByIndex(0));
        System.out.println("missing index: " + NetworkInterface.getByIndex(Integer.MAX_VALUE));
        InetAddress unbound = InetAddress.getByName("192.0.2.1");
        System.out.println("unbound address: " + NetworkInterface.getByInetAddress(unbound));
    }

    private static void describe(NetworkInterface networkInterface) throws Exception {
        System.out.println(networkInterface.getName() + " index=" + networkInterface.getIndex()
                + " displayName=" + networkInterface.getDisplayName());
        System.out.println("  addresses=" + Collections.list(networkInterface.getInetAddresses()));
        System.out.println("  interfaceAddresses=" + networkInterface.getInterfaceAddresses());
        System.out.println("  up=" + networkInterface.isUp() + " loopback=" + networkInterface.isLoopback()
                + " pointToPoint=" + networkInterface.isPointToPoint() + " virtual=" + networkInterface.isVirtual()
                + " multicast=" + networkInterface.supportsMulticast());
        System.out.println("  mtu=" + networkInterface.getMTU()
                + " hardwareAddress=" + Arrays.toString(networkInterface.getHardwareAddress()));
        System.out.println("  parent=" + networkInterface.getParent()
                + " subInterfaces=" + Collections.list(networkInterface.getSubInterfaces()));
        for (NetworkInterface subInterface : Collections.list(networkInterface.getSubInterfaces())) {
            describe(subInterface);
        }
    }
}
