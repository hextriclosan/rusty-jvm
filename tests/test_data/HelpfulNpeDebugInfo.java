package samples.npe.helpfulnpedebuginfo;

// Compiled with `javac -g` (see build.rs) so the class carries a LocalVariableTable. This exercises
// the JEP 358 message builder's source-name path: the null variable is named as written in source
// (e.g. `greeting`, `node`) rather than `<localN>`/`<parameterN>`.
public class HelpfulNpeDebugInfo {
    static class Node {
        Node next;
        int value;
        long total;
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

    static void writeFieldOnLocal() {
        Node node = null;
        node.value = 42;
    }

    // A two-slot value sits above the receiver on the operand stack.
    static void writeLongFieldOnParam(Node node) {
        node.total = 42L;
    }

    // The value is built by `new`, `dup` and a constructor call before the store faults.
    static void writeReferenceFieldOnLocal() {
        Node node = null;
        node.next = new Node();
    }

    public static void main(String[] args) {
        Runnable[] cases = {
                HelpfulNpeDebugInfo::invokeVirtualOnLocal,
                () -> readFieldOnParam(null),
                HelpfulNpeDebugInfo::readArrayLengthOnLocal,
                HelpfulNpeDebugInfo::loadFromArrayLocal,
                () -> invokeWithArgsReceiver("!"),
                () -> invokeAfterCast(null),
                HelpfulNpeDebugInfo::writeFieldOnLocal,
                () -> writeLongFieldOnParam(null),
                HelpfulNpeDebugInfo::writeReferenceFieldOnLocal,
        };
        String[] labels = {
                "invokeVirtualOnLocal",
                "readFieldOnParam",
                "readArrayLengthOnLocal",
                "loadFromArrayLocal",
                "invokeWithArgsReceiver",
                "invokeAfterCast",
                "writeFieldOnLocal",
                "writeLongFieldOnParam",
                "writeReferenceFieldOnLocal",
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
