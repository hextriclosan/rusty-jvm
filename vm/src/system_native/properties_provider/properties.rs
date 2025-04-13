use once_cell::sync::OnceCell;
use std::env;
use os_info::Type;

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

static OS_VERSION: OnceCell<String> = OnceCell::new();
pub(crate) fn os_version() -> &'static str {
    OS_VERSION.get_or_init(|| os_info::get().version().to_string())
}

static OS_NAME: OnceCell<&'static str> = OnceCell::new();
pub(crate) fn os_name() -> &'static str {
    OS_NAME.get_or_init(|| {
        let info = os_info::get();
        match info.os_type() {
            Type::AlmaLinux | Type::Alpaquita | Type::Alpine | Type::Amazon | Type::Arch |
            Type::Artix | Type::CachyOS | Type::CentOS | Type::Debian | Type::EndeavourOS |
            Type::Fedora | Type::Garuda | Type::Gentoo | Type::Kali |
            Type::Linux | Type::Mabox | Type::Manjaro | Type::Mariner |
            Type::Mint | Type::NixOS | Type::Nobara | Type::OpenCloudOS |
            Type::openEuler | Type::openSUSE | Type::OracleLinux | Type::Pop |
            Type::Raspbian | Type::Redhat | Type::RedHatEnterprise | Type::RockyLinux |
            Type::Solus | Type::SUSE | Type::Ubuntu | Type::Ultramarine |
            Type::Uos | Type::Void => "Linux",
            Type::Windows => "Windows",
            Type::Macos => "macOS",
            _ => unreachable!("Unsupported OS type"),
        }
    })
}

static USER_DIR: OnceCell<String> = OnceCell::new();
pub(crate) fn user_dir() -> &'static str {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    USER_DIR.get_or_init(|| current_dir.display().to_string())
}
