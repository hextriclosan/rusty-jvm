package samples.javacore.enums.tricky;

public class TrickyEnumsExample {

    public static void main(String[] args) {
        for (TrickyEnum te : TrickyEnum.values()) {
            printEnumValues(te);
        }
    }

    private static void printEnumValues(TrickyEnum trickyEnum) {
        System.out.println(trickyEnum.name() + " = " + trickyEnum.getLongName());
    }

}

enum TrickyEnum {
    ONE {
        @Override
        String getLongName() {
            return "I'm the ONE!";
        }
    }, TWO {
        @Override
        String getLongName() {
            return "I'm the Second!";
        }
    }, THREE {
        @Override
        String getLongName() {
            return "I'm a Loser";
        }
    };

    abstract String getLongName();
}
