package samples.system.archproperties;

public class ArchProperties {
    public static void main(String[] args) {
        System.out.println("os.arch=" + System.getProperty("os.arch"));
        System.out.println("sun.arch.data.model=" + System.getProperty("sun.arch.data.model"));
    }
}
