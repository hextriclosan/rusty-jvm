# rusty-jvm
![Platform](https://img.shields.io/badge/platforms-linux%20%7C%20macos%20%7C%20windows-blue)
[![Build Status](https://github.com/hextriclosan/rusty-jvm/actions/workflows/rust.yml/badge.svg)](https://github.com/hextriclosan/rusty-jvm/actions)
![LoC](https://tokei.rs/b1/github/hextriclosan/rusty-jvm?type=Rust)
![License](https://img.shields.io/github/license/hextriclosan/rusty-jvm)
[![codecov](https://codecov.io/gh/hextriclosan/rusty-jvm/branch/master/graph/badge.svg)](https://codecov.io/gh/hextriclosan/rusty-jvm)

## Introduction
This project is an attempt to implement JVM in Rust.
The project is for educational purposes only.
It is not intended to be a full-featured JVM implementation, but rather a simple one that can run small Java programs. 
The main goal of this project is to learn how JVM works and how to implement it in Rust.
There is no dependency on any existing JVM implementation, everything that regards java is implemented from scratch.
There is no garbage collection (yet, I hope).

The standard library classes are used in the project originated from _OpenJDK JDK 23 General-Availability Release_ are located in the [lib](lib) directory.

Refer [integration tests](tests/test_data) for examples of supported Java features.

## Getting Started

#### Prerequisites
Ensure you have the following:
- A machine running Windows, macOS, or Linux.
- Rust installed and configured.
- The `RUSTY_JAVA_HOME` environment variable set to the root of this project.

#### Example Program
This program calculates the total attack power of a group of units in a game. 
It features abstract classes, interfaces, and polymorphism to demonstrate the capabilities of rusty-jvm.
```java
package game;

import java.util.ArrayList;
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
        for (Unit unit : units) {
            this.units.add(unit);
        }
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
#### Steps to Run
1. Compile the program using the Java compiler
```shell
javac -d . Demo.java
```
2. Run it using rusty-jvm:
```shell
cargo run -- game.Demo
```
#### Expected Output
If everything is set up correctly, you should see
```
We embrace the glory of battle!
Battle is upon us!
Group attack power is 57
```
