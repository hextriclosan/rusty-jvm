package samples.inheritance.interfaceandabstractclass;

public class InterfaceAndAbstractClass {
    public static void main(String[] args) {
        Mercedes mercedes = new Mercedes(1, 10, 20, 100, 200);
        int mercedesResult = move(mercedes) + pullOver(mercedes);

        Vehicle vehicle = new Mercedes(10, 100, 200, 1000, 2000);
        int vehicleResult = move(vehicle) + pullOver(vehicle);

        Switchable switchable = new Mercedes(100, 1000, 2000, 10000, 20000);
        int switchableResult = move(switchable) + pullOver(switchable);

        int overallResult = mercedesResult + vehicleResult + switchableResult;
        System.out.println(overallResult);
    }

    private static int move(Switchable switchable) {
        return switchable.on();
    }

    private static int pullOver(Switchable switchable) {
        return switchable.off();
    }

}

interface Switchable {
    int on();
    int off();
}

abstract class Vehicle implements Switchable {
    final private int vehicleId;
    final private int basicPower;
    final private int basicBrakesQuality;

    public Vehicle(int vehicleId, int basicPower, int basicBrakesQuality) {
        this.vehicleId = vehicleId;
        this.basicPower = basicPower;
        this.basicBrakesQuality = basicBrakesQuality;
    }

    protected abstract int enginePower();
    protected abstract int brakesQuality();

    public int getVehicleId() {
        return vehicleId;
    }

    private int start() {
        return basicPower + enginePower();
    }

    private int stop() {
        return basicBrakesQuality + brakesQuality();
    }

    @Override
    public int on() {
        return start();
    }

    @Override
    public int off() {
        return stop();
    }

}

class Mercedes extends Vehicle {

    private final int maybachEnginePower;
    private final int akebonoBrakes;

    public Mercedes(int vehicleId, int basicPower, int basicBrakesQuality, int maybachEnginePower, int akebonoBrakes) {
        super(vehicleId, basicPower, basicBrakesQuality);
        this.maybachEnginePower = maybachEnginePower;
        this.akebonoBrakes = akebonoBrakes;
    }

    @Override
    protected int enginePower() {
        return maybachEnginePower;
    }

    @Override
    protected int brakesQuality() {
        return akebonoBrakes;
    }
}
