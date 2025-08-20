# rusty-jvm
![Platform][platforms-image]
[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][ci-image]][ci-link]
![License][license-image]
[![codecov][code-cov-image]][code-cov-link]
[![dependency status][dep-status-image]][dep-status-link]

## Introduction

This project is a Java Virtual Machine (JVM) implemented in Rust, built to run Java programs independently of existing JVMs.
Everything related to Java is implemented from scratch.
The current version executes Java bytecode in interpreted mode, with the introduction of a Just-In-Time (JIT) compiler identified as a future milestone. 
The next major objectives include the integration of garbage collection and support for multithreading.

## Implemented Key Features

- 100% of actual opcodes ([JVMS §6][jvms-6])
- Lambda Expressions ([JLS §15.27][jls-15.27])
- Static initialization ([JVMS §5.5][jvms-5.5])
- Polymorphic models ([JLS §8.4.8][jls-8.4.8])
- Single- and multi-dimensional arrays ([JLS §10][jls-10])
- Exceptions ([JLS §11][jls-11])
- Record Classes ([JVMS §8.10][jls-8.10])
- Type casting ([JLS §5.5][jls-5.5])
- Program arguments ([JLS §12.1.4][jls-12.1.4])
- Assertions ([JLS §14.10][jls-14.10])
- [Dynamic Language Support][java.lang.invoke-api] (partially)
- [Stream API][java.util.stream-api] (partially)
- [Reflection][java.lang.reflect-api] (some features)
- [java.io][java.io-api] (partially)
- [java.nio][java.nio-api] (partially)
- [java.util.zip][java.util.zip-api] (partially)
- [java.lang.System][java.lang.system-api] (most features)

See [integration tests](tests/test_data) for broader examples of supported Java features.

## Java Standard Library Classes

This project relies on standard library classes from the *OpenJDK JDK 23 General Availability Release*.
To run the Java code, you must have JDK 23 installed on your machine and ensure the `JAVA_HOME` environment variable is properly set.

## Getting Started

### Prerequisites

Ensure the following are set up:

- A machine running **Windows**, **macOS**, or **Linux**
- **JDK** installed and configured (OpenJDK 23 is recommended)
- **Rust** installed and configured

### Example Program

This Java program calculates the total attack power of a group of game units.  
It uses abstract classes, interfaces, and polymorphism to showcase rusty-jvm's capabilities.

```java
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
```

### Steps to Run

1. Compile the program using the Java compiler:
   ```sh
   javac -d . CompositePattern.java
   ```

2. Run it using rusty-jvm:
   ```sh
   cargo run -- samples.patterns.composite.CompositePattern
   ```

### Expected Output

If everything is set up correctly, you should see:

```
Assassin(100) says: Target acquired.
Assassin(110) says: Target acquired.
Soldier(2) says: For honor!
Army attack power is 222
```

## License
`rusty-jvm` is licensed under the [MIT License](LICENSE).

[//]: # (links)
[platforms-image]: https://img.shields.io/badge/platforms-linux%20%7C%20macos%20%7C%20windows-blue
[crate-image]: https://img.shields.io/crates/v/rusty-jvm.svg
[crate-link]: https://crates.io/crates/rusty-jvm
[docs-image]: https://docs.rs/rusty-jvm/badge.svg
[docs-link]: https://docs.rs/rusty-jvm
[ci-image]: https://github.com/hextriclosan/rusty-jvm/actions/workflows/rust.yml/badge.svg
[ci-link]: https://github.com/hextriclosan/rusty-jvm/actions
[license-image]: https://img.shields.io/github/license/hextriclosan/rusty-jvm
[code-cov-image]: https://codecov.io/gh/hextriclosan/rusty-jvm/branch/main/graph/badge.svg
[code-cov-link]: https://codecov.io/gh/hextriclosan/rusty-jvm
[dep-status-image]: https://deps.rs/repo/github/hextriclosan/rusty-jvm/status.svg
[dep-status-link]: https://deps.rs/repo/github/hextriclosan/rusty-jvm

[jvms-5.5]: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.5
[jvms-6]: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html
[jls-5.5]: https://docs.oracle.com/javase/specs/jls/se23/html/jls-5.html#jls-5.5
[jls-8.4.8]: https://docs.oracle.com/javase/specs/jls/se23/html/jls-8.html#jls-8.4.8
[jls-8.10]: https://docs.oracle.com/javase/specs/jls/se23/html/jls-8.html#jls-8.10
[jls-10]: https://docs.oracle.com/javase/specs/jls/se23/html/jls-10.html
[jls-11]: https://docs.oracle.com/javase/specs/jls/se23/html/jls-11.html
[jls-12.1.4]: https://docs.oracle.com/javase/specs/jls/se23/html/jls-12.html#jls-12.1.4
[jls-14.10]: https://docs.oracle.com/javase/specs/jls/se23/html/jls-14.html#jls-14.10
[jls-15.27]: https://docs.oracle.com/javase/specs/jls/se23/html/jls-15.html#jls-15.27
[java.util.stream-api]: https://docs.oracle.com/en/java/javase/23/docs/api/java.base/java/util/stream/package-summary.html
[java.io-api]: https://docs.oracle.com/en/java/javase/23/docs/api/java.base/java/io/package-summary.html
[java.nio-api]: https://docs.oracle.com/en/java/javase/23/docs/api/java.base/java/nio/package-summary.html
[java.lang.invoke-api]: https://docs.oracle.com/en/java/javase/23/docs/api/java.base/java/lang/invoke/package-summary.html
[java.lang.reflect-api]: https://docs.oracle.com/en/java/javase/23/docs/api/java.base/java/lang/reflect/package-summary.html
[java.util.zip-api]: https://docs.oracle.com/en/java/javase/23/docs/api/java.base/java/util/zip/package-summary.html
[java.lang.system-api]: https://docs.oracle.com/en/java/javase/23/docs/api/java.base/java/lang/System.html
