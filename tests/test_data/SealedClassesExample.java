package samples.reflection.sealed;

import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.util.Arrays;

public class SealedClassesExample {
    public static void main(String[] args) throws Exception {
        report(Shape.class);
        report(Shape.Circle.class);
        report(Square.class);
        report(Polygon.class);
        report(Hexagon.class);
        // permitted subclasses inferred from the compilation unit, as there is no permits clause
        report(Vehicle.class);
        report(Car.class);
        // an enum is implicitly sealed when a constant has a body
        report(Direction.class);
        report(Planet.class);
        report(SealedClassesExample.class);
        report(Shape[].class);
        report(int.class);

        // annotations are proxies, and Proxy checks that the annotation interface is not sealed
        Label label = SealedClassesExample.class.getDeclaredMethod("labelled").getAnnotation(Label.class);
        System.out.println("annotation: " + label.value());
    }

    private static void report(Class<?> clazz) {
        Class<?>[] permitted = clazz.getPermittedSubclasses();
        String permits = permitted == null
                ? "null"
                : Arrays.stream(permitted).map(SealedClassesExample::name).toList().toString();
        System.out.println(name(clazz) + ": sealed " + clazz.isSealed() + ", permits " + permits);
    }

    private static String name(Class<?> clazz) {
        return clazz.getName().replace(SealedClassesExample.class.getPackageName() + ".", "");
    }

    @Retention(RetentionPolicy.RUNTIME)
    @interface Label {
        String value();
    }

    @Label("labelled method")
    static void labelled() {
    }
}

sealed interface Shape permits Shape.Circle, Square, Polygon {
    record Circle(double radius) implements Shape {
    }
}

final class Square implements Shape {
}

non-sealed class Polygon implements Shape {
}

class Hexagon extends Polygon {
}

abstract sealed class Vehicle {
}

final class Car extends Vehicle {
}

final class Truck extends Vehicle {
}

enum Direction {
    NORTH {
        @Override
        public String toString() {
            return "up";
        }
    },
    SOUTH
}

enum Planet {
    MERCURY,
    VENUS
}
