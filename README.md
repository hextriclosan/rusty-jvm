# rusty-jvm
![Platform][platforms-image]
[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][ci-image]][ci-link]
![License][license-image]
[![codecov][code-cov-image]][code-cov-link]
[![dependency status][dep-status-image]][dep-status-link]

## Introduction

Writing a JVM has long been a dream of mine, so I decided to give it a try combining my curiosity about how suitable Rust is for such a task with my desire to see how far I could push it.
I'm not the first to explore this idea [this project][rjvm-articles] is a well-known and easily searchable example but unlike that project, I aim to create a JVM capable of running as much Java code as possible.
I have to say, I didn’t expect to get this far.
Java code is run in interpreted mode, meaning the JVM reads and executes bytecode instructions directly without compiling them to native code, so please don't expect high performance.
There is no dependency on any existing JVM implementation. Everything related to Java is implemented from scratch.
One major feature that’s still missing is garbage collection. That’s the next big milestone.

## Implemented Key Features

- 99% of actual opcodes ([JVMS §6][jvms-6])
- Static initialization ([JVMS §5.5][jvms-5.5])
- Polymorphic models ([JLS §8.4.8][jls-8.4.8])
- Single- and multi-dimensional arrays ([JLS §10][jls-10])
- Exceptions ([JLS §11][jls-11])
- [java.io][java.io-api] (partially)
- [java.nio][java.nio-api] (partially)
- Type casting ([JLS §5.5][jls-5.5])
- Program arguments ([JLS §12.1.4][jls-12.1.4])
- [java.lang.System][java.lang.system-api] (most features)
- [Reflection][java.lang.reflect-api] (some features)
- Assertions ([JLS §14.10][jls-14.10])

See [integration tests](tests/test_data) for broader examples of supported Java features.

## Java Standard Library Classes

This project uses Java standard library classes based on *OpenJDK JDK 23 General-Availability Release*.
The source code and class files are located in the [lib](lib) directory of this repository.  
They are distributed under the [GNU GPLv2 with Classpath Exception](lib/LICENSE).
OpenJDK produces OS-specific distributions of these classes.  
For simplicity, this project includes modified (under GPLv2+CE) versions usable across **Windows**, **macOS**, and **Linux**.
You can still use original OpenJDK classes, but ensure they're compiled for the OS on which you're running `rusty-jvm`.

## Getting Started

### Prerequisites

Ensure the following are set up:

- A machine running **Windows**, **macOS**, or **Linux**
- **Rust** installed and configured
- The **Java standard library classes**, required via **one** of the following:
    - Run the installation command: `rusty-jvm --install`
    - **OR** set the `RUSTY_LIB_DIR` environment variable to the path of the standard library  
      (recommended: the [lib](lib) directory in this repository)

### Example Program

This Java program calculates the total attack power of a group of game units.  
It uses abstract classes, interfaces, and polymorphism to showcase rusty-jvm's capabilities.

```java
package game;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Demo {
    public static void main(String[] args) {
        ControlGroup controlGroup = new ControlGroup();
        controlGroup.addUnits(new Zealot(), new DarkTemplar(), new Unit() {
            @Override
            public int damage() {
                return 4;
            }
        });

        int groupAttackPower = controlGroup.damage();
        System.out.print("Group attack power is ");
        System.out.println(groupAttackPower);
    }
}

interface Unit {
    int damage();
}

abstract class AbstractUnit implements Unit {
    public AbstractUnit() {
        System.out.println(say());
    }
    protected abstract String say();
}

class Zealot extends AbstractUnit {
    @Override
    public int damage() {
        return 8;
    }
    @Override
    public String say() {
        return "We embrace the glory of battle!";
    }
}

class DarkTemplar extends AbstractUnit {
    @Override
    public int damage() {
        return 45;
    }
    @Override
    public String say() {
        return "Battle is upon us!";
    }
}

class ControlGroup implements Unit {
    private final List<Unit> units = new ArrayList<>();
    public void addUnits(Unit... units) {
        this.units.addAll(Arrays.asList(units));
    }
    @Override
    public int damage() {
        int totalAttackPower = 0;
        for (Unit unit : units) {
            totalAttackPower += unit.damage();
        }
        return totalAttackPower;
    }
}
```

### Steps to Run

1. Compile the program using the Java compiler:
   ```sh
   javac -d . Demo.java
   ```

2. Run it using rusty-jvm:
   ```sh
   cargo run -- game.Demo
   ```

### Expected Output

If everything is set up correctly, you should see:

```
We embrace the glory of battle!
Battle is upon us!
Group attack power is 57
```

## License
`rusty-jvm` is licensed under the [MIT License](LICENSE).

Standard library classes in [lib](lib) directory are distributed under the [GNU GPLv2 with Classpath Exception](lib/LICENSE).

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

[rjvm-articles]: https://andreabergia.com/series/writing-a-jvm-in-rust/

[jvms-5.5]: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.5
[jvms-6]: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html
[jls-5.5]: https://docs.oracle.com/javase/specs/jls/se23/html/jls-5.html#jls-5.5
[jls-8.4.8]: https://docs.oracle.com/javase/specs/jls/se23/html/jls-8.html#jls-8.4.8
[jls-10]: https://docs.oracle.com/javase/specs/jls/se23/html/jls-10.html
[jls-11]: https://docs.oracle.com/javase/specs/jls/se23/html/jls-11.html
[jls-12.1.4]: https://docs.oracle.com/javase/specs/jls/se23/html/jls-12.html#jls-12.1.4
[jls-14.10]: https://docs.oracle.com/javase/specs/jls/se23/html/jls-14.html#jls-14.10
[java.io-api]: https://docs.oracle.com/en/java/javase/23/docs/api/java.base/java/io/package-summary.html
[java.nio-api]: https://docs.oracle.com/en/java/javase/23/docs/api/java.base/java/nio/package-summary.html
[java.lang.system-api]: https://docs.oracle.com/en/java/javase/23/docs/api/java.base/java/lang/System.html
[java.lang.reflect-api]: https://docs.oracle.com/en/java/javase/23/docs/api/java.base/java/lang/reflect/package-summary.html
