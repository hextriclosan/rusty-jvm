// build command:
// javac -g -parameters Shape.java

public sealed interface Shape permits Circle {

}

final class Circle implements Shape {

}
