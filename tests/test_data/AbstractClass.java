package samples.inheritance.abstractclass;

public class AbstractClass {
    public static void main(String[] args) {
        Mercedes vehicle = new Mercedes(10, 100, 17);
        int fullPower = vehicle.fullPower();
        System.out.println(fullPower);
    }
}

abstract class Vehicle {
    final protected int basicPower;
    final private int defaultPower;

    public Vehicle(int basicPower, int defaultPower) {
        this.basicPower = basicPower;
        this.defaultPower = defaultPower;
    }

    public int fullPower() {
        return basicPower + enginePower() + defaultPower;
    }

    protected abstract int enginePower();
}

class Mercedes extends Vehicle {

    private final int maybachEnginePower;
    final private int defaultPower;

    public Mercedes(int basicPower, int maybachEnginePower, int defaultPower) {
        super(basicPower, defaultPower);
        this.maybachEnginePower = maybachEnginePower;
        this.defaultPower = defaultPower / 2;
    }

    @Override
    protected int enginePower() {
        return basicPower + maybachEnginePower + defaultPower;
    }
}
