package samples.inheritance.interfaces.compositepattern;

import java.util.ArrayList;
import java.util.List;

public class CompositePattern {
    public static void main(String[] args) {
        Composite outerComposite = new Composite();
        outerComposite.addUnit(new Zealot());
        outerComposite.addUnit(new Zealot());
        outerComposite.addUnit(new DarkTemplar());
        outerComposite.addUnit(new DarkTemplar());

        Composite innerComposite = new Composite();
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
        System.out.println(result);
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

    private final List<Unit> units;

    public Composite() {
        units = new ArrayList<>();
    }

    public void addUnit(Unit unit) {
        units.add(unit);
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
