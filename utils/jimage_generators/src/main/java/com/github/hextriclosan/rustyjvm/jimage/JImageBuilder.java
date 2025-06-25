package com.github.hextriclosan.rustyjvm.jimage;

import picocli.CommandLine;

public class JImageBuilder {
    public static void main(String[] args) {
        int exitCode = new CommandLine(new ImageCreator()).execute(args);
        System.exit(exitCode);
    }
}
