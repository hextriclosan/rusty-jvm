package samples.system.userproperties;

public class UserProperties {
    public static void main(String[] args) {
        System.out.println("user.name=" + System.getProperty("user.name"));
        System.out.println("user.home=" + System.getProperty("user.home"));
    }
}
