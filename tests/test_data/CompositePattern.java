package samples.patterns.composite;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Objects;
import java.util.Optional;

public class CompositePattern {
    public static void main(String[] args) {
        UnitGroup eliteSquad = new UnitGroup(new Assassin(50));
        eliteSquad.addUnits(new Assassin(55));
        Unit namelessSquad = new UnitGroup(
                () -> 2,
                new AbstractUnit(7) {
                },
                new Soldier("Soldier", "For honor!", 11));

        Unit army = new UnitGroup(eliteSquad, namelessSquad);
        System.out.println("Army attack power is " + army.damage());
    }
}

interface Unit {
    int damage();
}

interface TalkingUnit extends Unit {
    int damageValue();

    default int attack() {
        System.out.println(this);
        return damageValue();
    }

    @Override
    default int damage() {
        return attack();
    }
}

abstract class AbstractUnit implements TalkingUnit {
    private final String name;
    private final String phrase;
    private final int damageValue;

    protected AbstractUnit(String name, String phrase, int damageValue) {
        this.name = Optional.ofNullable(name).orElseGet(this::getDefaultName);
        this.phrase = Objects.requireNonNullElse(phrase, "Arrr!");
        this.damageValue = damageValue;
    }

    protected AbstractUnit(int damageValue) {
        this(null, null, damageValue);
    }

    private String getDefaultName() {
        return Optional.of(getClass().getSimpleName())
                .filter(s -> !s.isBlank())
                .orElse("Unknown Unit");
    }

    @Override
    public int damageValue() {
        return damageValue;
    }

    @Override
    public String toString() {
        return name + "(" + damageValue() + ") says: " + phrase;
    }
}

class Assassin extends AbstractUnit {
    public Assassin(int damageValue) {
        super(null, "Target acquired.", damageValue);
    }

    @Override
    public int damageValue() {
        return super.damageValue() * 2; // Assassins deal extra damage
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
        return units.stream()
                .mapToInt(Unit::damage)
                .sum();
    }
}

record Soldier(String name, String phrase, int damageValue) implements TalkingUnit {
}
