use os_info::Type;
use os_info::Version::Semantic;
use std::env;
use std::sync::LazyLock;

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

pub(crate) fn path_separator() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        ";"
    }
    #[cfg(not(target_os = "windows"))]
    {
        ":"
    }
}

pub(crate) fn file_separator() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "\\"
    }
    #[cfg(not(target_os = "windows"))]
    {
        "/"
    }
}

static OS_VERSION: LazyLock<String> = LazyLock::new(|| {
    let info = os_info::get();
    let version = info.version();
    match version {
        Semantic(_, _, _) => version.to_string(),
        _ => "0.0.0".to_string(),
    }
});
pub(crate) fn os_version() -> &'static str {
    &OS_VERSION
}

static OS_NAME: LazyLock<&'static str> = LazyLock::new(|| {
    let info = os_info::get();
    match info.os_type() {
        Type::AlmaLinux
        | Type::Alpaquita
        | Type::Alpine
        | Type::Amazon
        | Type::Arch
        | Type::Artix
        | Type::CachyOS
        | Type::CentOS
        | Type::Debian
        | Type::EndeavourOS
        | Type::Fedora
        | Type::Garuda
        | Type::Gentoo
        | Type::Kali
        | Type::Linux
        | Type::Mabox
        | Type::Manjaro
        | Type::Mariner
        | Type::Mint
        | Type::NixOS
        | Type::Nobara
        | Type::OpenCloudOS
        | Type::openEuler
        | Type::openSUSE
        | Type::OracleLinux
        | Type::Pop
        | Type::Raspbian
        | Type::Redhat
        | Type::RedHatEnterprise
        | Type::RockyLinux
        | Type::Solus
        | Type::SUSE
        | Type::Ubuntu
        | Type::Ultramarine
        | Type::Uos
        | Type::Void => "Linux",
        Type::Windows => "Windows",
        Type::Macos => "macOS",
        _ => unreachable!("Unsupported OS type"),
    }
});
pub(crate) fn os_name() -> &'static str {
    &OS_NAME
}

static USER_DIR: LazyLock<String> = LazyLock::new(|| {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    current_dir.display().to_string()
});
pub(crate) fn user_dir() -> &'static str {
    &USER_DIR
}

static TMP_DIR: LazyLock<String> = LazyLock::new(|| env::temp_dir().display().to_string());
pub(crate) fn tmp_dir() -> &'static str {
    &TMP_DIR
}
