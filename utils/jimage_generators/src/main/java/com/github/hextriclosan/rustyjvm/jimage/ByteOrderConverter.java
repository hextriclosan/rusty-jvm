package com.github.hextriclosan.rustyjvm.jimage;

import picocli.CommandLine;

import java.nio.ByteOrder;

public class ByteOrderConverter implements CommandLine.ITypeConverter<ByteOrder> {
    @Override
    public ByteOrder convert(String value) {
        return switch (value.toLowerCase()) {
            case "big" -> ByteOrder.BIG_ENDIAN;
            case "little" -> ByteOrder.LITTLE_ENDIAN;
            default ->
                    throw new CommandLine.TypeConversionException("Invalid byte order: " + value + ". Use 'big' or 'little'.");
        };
    }
}

