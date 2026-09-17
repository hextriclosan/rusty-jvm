use crate::vm::error::Error;
use crate::vm::JAVA_HOME;
use os_info::Type;
use os_info::Version::Semantic;
use std::env;
#[cfg(all(unix, not(target_os = "macos")))]
use std::ffi::CStr;
use std::string::ToString;
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

#[cfg(windows)]
fn windows_encoding(code_page: u32) -> String {
    match code_page {
        0 | 65001 => "UTF-8".to_string(),
        874 | 932 | 936 | 949 | 950 | 1361 => format!("MS{code_page}"),
        54936 => "GB18030".to_string(),
        _ => format!("Cp{code_page}"),
    }
}

#[cfg(windows)]
static SUN_JNU_ENCODING: LazyLock<String> =
    LazyLock::new(|| windows_encoding(unsafe { winapi::um::winnls::GetACP() }));

#[cfg(target_os = "macos")]
static SUN_JNU_ENCODING: LazyLock<String> = LazyLock::new(|| "UTF-8".to_string());

#[cfg(all(unix, not(target_os = "macos")))]
static SUN_JNU_ENCODING: LazyLock<String> = LazyLock::new(|| unsafe {
    if nix::libc::setlocale(nix::libc::LC_CTYPE, c"".as_ptr()).is_null() {
        return "ISO8859-1".to_string();
    }

    let encoding = nix::libc::nl_langinfo(nix::libc::CODESET);
    if encoding.is_null() {
        return "ISO8859-1".to_string();
    }

    match CStr::from_ptr(encoding).to_string_lossy().as_ref() {
        "" => "ISO8859-1".to_string(),
        "646" => "ISO646-US".to_string(),
        "EUC-JP" => "EUC-JP-LINUX".to_string(),
        encoding => encoding.to_string(),
    }
});

pub(crate) fn sun_jnu_encoding() -> &'static str {
    &SUN_JNU_ENCODING
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
static JAVA_HOME_PROP: LazyLock<String> = LazyLock::new(|| {
    JAVA_HOME
        .get()
        .expect("JAVA_HOME is not initialized")
        .to_str()
        .ok_or_else(|| {
            let invalid_path = JAVA_HOME.get().unwrap().as_os_str().to_string_lossy();
            let msg = format!("Failed to convert JAVA_HOME to UTF-8: {:?}", invalid_path);
            Error::new_execution(&msg)
        })
        .unwrap()
        .to_string()
});
pub(crate) fn java_home() -> &'static str {
    &JAVA_HOME_PROP
}

static SUN_BOOT_LIBRARY_PATH: LazyLock<String> = LazyLock::new(|| {
    let dir = if cfg!(target_os = "windows") {
        "bin"
    } else {
        "lib"
    };
    format!("{}{}{}", &*JAVA_HOME_PROP, file_separator(), dir)
});
pub(crate) fn sun_boot_library_path() -> &'static str {
    &SUN_BOOT_LIBRARY_PATH
}

#[cfg(all(test, windows))]
mod tests {
    use super::windows_encoding;

    #[test]
    fn should_map_windows_code_pages_to_java_charset_names() {
        assert_eq!(windows_encoding(0), "UTF-8");
        assert_eq!(windows_encoding(65001), "UTF-8");
        assert_eq!(windows_encoding(932), "MS932");
        assert_eq!(windows_encoding(54936), "GB18030");
        assert_eq!(windows_encoding(1251), "Cp1251");
    }
}
