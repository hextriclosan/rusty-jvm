package samples.patterns.composite;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Composite {
    public static void main(String[] args) {
        Unit eliteSquad = new UnitGroup(new Assassin(), new Archer());
        Unit namelessSquad = new UnitGroup(
                new Unit() {
                    @Override
                    public int damage() {
                        return 2;
                    }
                },
                new AbstractUnit() {
                    @Override
                    protected String phrase() {
                        return "Where am I?";
                    }

                    @Override
                    public int damage() {
                        return 1;
                    }
                });

        Unit army = new UnitGroup(eliteSquad, namelessSquad);
        System.out.println("Army attack power is " + army.damage());
    }
}

interface Unit {
    int damage();
}

abstract class AbstractUnit implements Unit {
    public AbstractUnit() {
        System.out.println(getIntroMessage());
    }

    protected String getName() {
        String name = getClass().getSimpleName();
        return !name.isEmpty() ? name : "Unnamed Unit";
    }

    protected abstract String phrase();

    private String getIntroMessage() {
        return getName() + ": " + phrase();
    }
}

class Assassin extends AbstractUnit {
    @Override
    public int damage() {
        return 45;
    }

    @Override
    protected String phrase() {
        return "Target acquired.";
    }
}

class Archer extends AbstractUnit {
    @Override
    public int damage() {
        return 12;
    }

    @Override
    protected String phrase() {
        return "Ready to fire.";
    }
}

class UnitGroup implements Unit {
    private final List<Unit> units = new ArrayList<>();

    public UnitGroup(Unit... units) {
        addUnits(units);
    }

    public void addUnits(Unit... units) {
        this.units.addAll(Arrays.asList(units));
    }

    @Override
    public int damage() {
        int totalDamage = 0;
        for (Unit unit : units) {
            totalDamage += unit.damage();
        }
        return totalDamage;
    }
}
