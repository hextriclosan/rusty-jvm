use once_cell::sync::OnceCell;
use std::env;
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

static OS_INFO: OnceCell<String> = OnceCell::new();
pub(crate) fn os_version() -> &'static str {
    OS_INFO.get_or_init(|| os_info::get().version().to_string())
}

static USER_DIR: OnceCell<String> = OnceCell::new();
pub(crate) fn user_dir() -> &'static str {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    USER_DIR.get_or_init(|| current_dir.display().to_string())
}
