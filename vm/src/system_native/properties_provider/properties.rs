pub(crate) fn is_bigendian() -> bool {
    #[cfg(target_endian = "big")]
    {
        true
    }
    #[cfg(not(target_endian = "big"))]
    {
        false
    }
}

pub(crate) fn endianness() -> &'static str {
    #[cfg(target_endian = "big")]
    {
        "big"
    }
    #[cfg(not(target_endian = "big"))]
    {
        "little"
    }
}

pub(crate) fn line_separator() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "\r\n"
    }
    #[cfg(not(target_os = "windows"))]
    {
        "\n"
    }
}
