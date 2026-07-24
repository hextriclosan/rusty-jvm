package samples.npe.helpfulnpedebuginfo;

// Compiled with `javac -g` (see build.rs) so the class carries a LocalVariableTable. This exercises
// the JEP 358 message builder's source-name path: the null variable is named as written in source
// (e.g. `greeting`, `node`) rather than `<localN>`/`<parameterN>`.
public class HelpfulNpeDebugInfo {
    static class Node {
        Node next;
        int value;
    }

    static void invokeVirtualOnLocal() {
        String greeting = null;
        System.out.println(greeting.length());
    }

    static void readFieldOnParam(Node node) {
        System.out.println(node.value);
    }

    static void readArrayLengthOnLocal() {
        int[] numbers = null;
        System.out.println(numbers.length);
    }

    static void loadFromArrayLocal() {
        String[] names = null;
        System.out.println(names[0]);
    }

    static void invokeWithArgsReceiver(String prefix) {
        String target = null;
        System.out.println(target.concat(prefix));
    }

    static void invokeAfterCast(Object raw) {
        System.out.println(((CharSequence) raw).length());
    }

    public static void main(String[] args) {
        Runnable[] cases = {
                HelpfulNpeDebugInfo::invokeVirtualOnLocal,
                () -> readFieldOnParam(null),
                HelpfulNpeDebugInfo::readArrayLengthOnLocal,
                HelpfulNpeDebugInfo::loadFromArrayLocal,
                () -> invokeWithArgsReceiver("!"),
                () -> invokeAfterCast(null),
        };
        String[] labels = {
                "invokeVirtualOnLocal",
                "readFieldOnParam",
                "readArrayLengthOnLocal",
                "loadFromArrayLocal",
                "invokeWithArgsReceiver",
                "invokeAfterCast",
        };
        for (int i = 0; i < cases.length; i++) {
            try {
                cases[i].run();
            } catch (Throwable t) {
                System.out.println(labels[i] + ": " + t.getMessage());
            }
        }
    }
}
