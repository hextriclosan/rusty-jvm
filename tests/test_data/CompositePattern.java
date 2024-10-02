package samples.inheritance.interfaces.compositepattern;

public class CompositePattern {
    public static void main(String[] args) {
        Composite outerComposite = new Composite(5);
        outerComposite.addUnit(new Zealot());
        outerComposite.addUnit(new Zealot());
        outerComposite.addUnit(new DarkTemplar());
        outerComposite.addUnit(new DarkTemplar());

        Composite innerComposite = new Composite(3);
        innerComposite.addUnit(new Zealot());
        innerComposite.addUnit(new Unit() {
            @Override
            public int attack() {
                return 7;
            }
        });
        innerComposite.addUnit(new DarkTemplar());

        outerComposite.addUnit(innerComposite);

        int result = outerComposite.attack();
    }
}

interface Unit {
    int attack();
}

class Zealot implements Unit {
    @Override
    public int attack() {
        return 11;
    }
}

class DarkTemplar implements Unit {
    @Override
    public int attack() {
        return 220;
    }
}

class Composite implements Unit {

    private final Unit[] units;
    private int currentIndex;

    public Composite(int size) {
        units = new Unit[size];
        currentIndex = 0;
    }

    public void addUnit(Unit unit) {
        units[currentIndex++] = unit;
    }

    @Override
    public int attack() {
        int attackPower = 0;

        for (Unit unit : units) {
            if (unit != null) {
                attackPower += unit.attack();
            }
        }

        return attackPower;
    }
}
