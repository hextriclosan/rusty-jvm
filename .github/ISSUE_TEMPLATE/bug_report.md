---
name: Bug report
about: Something in the VM behaves incorrectly or crashes
title: '[Bug] '
labels: bug
assignees: ''
---

## Description

<!-- A clear and concise description of the bug. -->

## Java program that reproduces it

```java
public class Repro {
    public static void main(String[] args) {
        // minimal reproduction
    }
}
```

## Steps to reproduce

1. Compile: `javac Repro.java`
2. Run: `cargo run -- Repro`

## Expected behaviour

<!-- What should happen. -->

## Actual behaviour

<!-- What actually happens. Include the full error message / stack trace. -->

## Environment

| Field | Value |
|---|---|
| OS | <!-- e.g. Ubuntu 24.04 x86_64 / macOS 14 arm64 / Windows 11 --> |
| Rust version | <!-- `rustc --version` --> |
| JDK version | <!-- `java -version` --> |
| rusty-jvm version | <!-- `cargo pkgid rusty-jvm` or git commit SHA --> |

## Additional context

<!-- Any other information that might be relevant (e.g. does it work with a real JVM?). -->
