package samples.system.normaloutputwitherroutput;

public class NormalOutputWithErrorOutput {
    public static void main(String[] args) {
        System.out.println("This is normal output.");
        System.err.println("This is error output.");
        System.out.println("This is another normal output.");
        System.err.println("This is another error output.");
    }
}
