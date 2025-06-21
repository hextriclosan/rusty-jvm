package com.github.hextriclosan.rustyjvm.jimage;

import jdk.internal.util.Architecture;
import jdk.internal.util.OperatingSystem;
import jdk.tools.jlink.builder.DefaultImageBuilder;
import jdk.tools.jlink.builder.ImageBuilder;
import jdk.tools.jlink.internal.Archive;
import jdk.tools.jlink.internal.DirArchive;
import jdk.tools.jlink.internal.ImageFileCreator;
import jdk.tools.jlink.internal.ImagePluginStack;
import jdk.tools.jlink.internal.Platform;

import java.io.IOException;
import java.nio.file.Paths;
import java.util.Collections;
import java.util.List;
import java.util.Set;

public class JImageBuilder {
    public static void main(String[] args) throws IOException {
        ImageBuilder builder = new DefaultImageBuilder(
                Paths.get("../../jimage-rs/tests/test_data/jimages"),
                Collections.emptyMap(),
                new Platform(OperatingSystem.WINDOWS, Architecture.X64));

        Set<Archive> archives = Set.of(
                new DirArchive(Paths.get("../../jimage-rs/tests/test_data/mods/java.base"), "java.base"));

        ImagePluginStack imagePluginStack = new ImagePluginStack(builder, List.of(), null);
        ImageFileCreator.create(archives, imagePluginStack);
    }
}
