package samples.reflection.trivial.classisinterface;

import java.lang.annotation.ElementType;
import java.util.AbstractMap;
import java.util.HashMap;

public class ClassIsInterfaceExample {
    public static void main(String[] args) {
        boolean runnableIsInterface = Runnable.class.isInterface();
        boolean hashMapIsInterface = HashMap.class.isInterface();
        boolean abstractMapIsInterface = AbstractMap.class.isInterface();
        boolean elementTypeIsInterface = ElementType.class.isInterface();

        int result = runnableIsInterface && !hashMapIsInterface && !abstractMapIsInterface && !elementTypeIsInterface
                ? 1
                : 0;
    }
}
