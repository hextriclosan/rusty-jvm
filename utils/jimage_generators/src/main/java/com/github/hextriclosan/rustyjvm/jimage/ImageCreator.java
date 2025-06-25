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
import jdk.tools.jlink.internal.plugins.DefaultCompressPlugin;
import jdk.tools.jlink.plugin.Plugin;
import picocli.CommandLine;

import java.nio.ByteOrder;
import java.nio.file.Paths;
import java.util.Collections;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.concurrent.Callable;

@CommandLine.Command(
        mixinStandardHelpOptions = true,
        version = "JImageBuilder 1.0",
        description = "Builds a jimage file with specified byte order and compression options.")
public class ImageCreator implements Callable<Integer> {
    @CommandLine.Option(
            names = "--endianness",
            description = "Byte order: big or little",
            converter = ByteOrderConverter.class,
            defaultValue = "little",
            required = true)
    private ByteOrder endianness;

    @CommandLine.Option(
            names = "--compress",
            defaultValue = "false",
            description = "Enable compression")
    private boolean compress;

    @Override
    public Integer call() throws Exception {
        ImageBuilder builder = new DefaultImageBuilder(
                Paths.get("../../jimage-rs/tests/test_data"),
                Collections.emptyMap(),
                new Platform(OperatingSystem.WINDOWS, Architecture.X64));

        Set<Archive> archives = Set.of(
                new DirArchive(Paths.get("../../jimage-rs/tests/test_data/mods/java.base"), "java.base"));

        var plugins = plugins(compress);
        ImagePluginStack imagePluginStack = new ImagePluginStack(builder, plugins, null);
        ImageFileCreator.create(archives, endianness, imagePluginStack);

        System.out.println("JImage file created successfully with " + endianness + " byte order and compression " + (compress ? "enabled" : "disabled") + ".");
        return 0;
    }

    private static List<Plugin> plugins(boolean compressed) {
        if (!compressed) {
            return Collections.emptyList();
        }

        DefaultCompressPlugin defaultCompressPlugin = new DefaultCompressPlugin();
        defaultCompressPlugin.configure(Map.of("compress", "zip-9"));
        return List.of(defaultCompressPlugin);
    }
}
