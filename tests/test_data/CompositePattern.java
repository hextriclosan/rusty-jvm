package samples.patterns.composite;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class CompositePattern {
    public static void main(String[] args) {
        Unit eliteSquad = new UnitGroup(
                new Assassin(50),
                new Assassin(55));
        Unit namelessSquad = new UnitGroup(
                () -> 10,
                new Soldier("Soldier", "For honor!", 2));

        Unit army = new UnitGroup(eliteSquad, namelessSquad);
        System.out.println("Army attack power is " + army.damage());
    }
}

interface Unit {
    int damage();
}

interface TalkingUnit extends Unit {
    String name();

    String phrase();

    int damageValue();

    @Override
    default int damage() {
        int dmg = damageValue();
        speak(dmg);
        return dmg;
    }

    default void speak(int dmg) {
        System.out.println(name() + "(" + dmg + ") says: " + phrase());
    }
}

abstract class AbstractTalkingUnit implements TalkingUnit {
    private final String name;
    private final String phrase;
    private final int baseDamage;

    protected AbstractTalkingUnit(String name, String phrase, int baseDamage) {
        this.name = name != null ? name : getClass().getSimpleName();
        this.phrase = phrase != null ? phrase : "Arrr!";
        this.baseDamage = baseDamage;
    }

    @Override
    public int damageValue() {
        return baseDamage * damageMultiplier();
    }

    @Override
    public String phrase() {
        return phrase;
    }

    @Override
    public String name() {
        return name;
    }

    protected abstract int damageMultiplier();
}

class Assassin extends AbstractTalkingUnit {
    public Assassin(int damageValue) {
        super(null, "Target acquired.", damageValue);
    }

    @Override
    protected int damageMultiplier() {
        return 2;
    }
}

class UnitGroup implements Unit {
    private final List<Unit> units = new ArrayList<>();

    public UnitGroup(Unit... units) {
        this.units.addAll(Arrays.asList(units));
    }

    @Override
    public int damage() {
        return units.stream()
                .mapToInt(Unit::damage)
                .sum();
    }
}

record Soldier(String name, String phrase, int damageValue) implements TalkingUnit {
}
