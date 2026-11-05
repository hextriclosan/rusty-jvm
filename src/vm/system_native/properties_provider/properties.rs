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

pub(crate) fn unicode_encoding() -> &'static str {
    // HotSpot tests `BYTE_ORDER == _LITTLE_ENDIAN` on macOS, which lacks `_LITTLE_ENDIAN`, so
    // it reports UnicodeBig there despite the little-endian CPUs
    if is_bigendian() || cfg!(target_os = "macos") {
        "UnicodeBig"
    } else {
        "UnicodeLittle"
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
        950 if windows_has_hkscs() => "MS950_HKSCS_XP".to_string(),
        936 if windows_has_gb18030() => "GB18030".to_string(),
        874 | 932 | 936 | 949 | 950 | 1361 => format!("MS{code_page}"),
        54936 => "GB18030".to_string(),
        _ => format!("Cp{code_page}"),
    }
}

/// Traditional Chinese Windows with the HKSCS patch maps MS950 0xFA41 to U+92DB instead of U+E001
#[cfg(windows)]
fn windows_has_hkscs() -> bool {
    use winapi::um::stringapiset::MultiByteToWideChar;
    use winapi::um::winnls::CP_ACP;

    let mb_char = [0xFAu8 as i8, 0x41];
    let mut unicode_char = 0u16;
    let converted =
        unsafe { MultiByteToWideChar(CP_ACP, 0, mb_char.as_ptr(), 2, &mut unicode_char, 1) };
    converted != 0 && unicode_char == 0x92DB
}

/// Simplified Chinese Windows with the GB18030 patch has code page 54936 and the SimSun18030 font
#[cfg(windows)]
fn windows_has_gb18030() -> bool {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use std::path::PathBuf;
    use winapi::shared::minwindef::MAX_PATH;
    use winapi::um::sysinfoapi::GetWindowsDirectoryW;
    use winapi::um::winnls::IsValidCodePage;

    if unsafe { IsValidCodePage(54936) } == 0 {
        return false;
    }
    let mut buffer = [0u16; MAX_PATH + 1];
    let len = unsafe { GetWindowsDirectoryW(buffer.as_mut_ptr(), buffer.len() as u32) } as usize;
    if len == 0 || len > buffer.len() {
        return false;
    }
    PathBuf::from(OsString::from_wide(&buffer[..len]))
        .join("Fonts")
        .join("SimSun18030.ttc")
        .is_file()
}

#[cfg(windows)]
static SUN_JNU_ENCODING: LazyLock<String> =
    LazyLock::new(|| windows_encoding(unsafe { winapi::um::winnls::GetACP() }));

#[cfg(target_os = "macos")]
static SUN_JNU_ENCODING: LazyLock<String> = LazyLock::new(|| "UTF-8".to_string());

#[cfg(all(unix, not(target_os = "macos")))]
static SUN_JNU_ENCODING: LazyLock<String> = LazyLock::new(|| unsafe {
    // like OpenJDK, ignore a failed activation: the still-active locale (e.g. "C") gives the codeset
    nix::libc::setlocale(nix::libc::LC_ALL, c"".as_ptr());

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

/// The JDK build names the CPU for `os.arch` (`OPENJDK_TARGET_CPU_OSARCH`), and Windows knows
/// only `amd64` and `aarch64`
pub(crate) fn os_arch() -> &'static str {
    if cfg!(windows) {
        return match env::consts::ARCH {
            "x86_64" => "amd64",
            "aarch64" => "aarch64",
            _ => "unknown",
        };
    }
    match env::consts::ARCH {
        "x86_64" if cfg!(target_pointer_width = "32") => "x32",
        "x86_64" if cfg!(target_os = "macos") => "x86_64",
        "x86_64" => "amd64",
        "x86" if cfg!(target_os = "linux") => "i386",
        "powerpc" => "ppc",
        "powerpc64" if cfg!(target_endian = "little") => "ppc64le",
        "powerpc64" => "ppc64",
        "mips" if cfg!(target_endian = "little") => "mipsel",
        "mips64" if cfg!(target_endian = "little") => "mips64el",
        "sparc64" => "sparcv9",
        architecture => architecture,
    }
}

pub(crate) fn data_model() -> &'static str {
    if cfg!(target_pointer_width = "64") {
        "64"
    } else {
        "32"
    }
}

static USER_DIR: LazyLock<String> = LazyLock::new(|| {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    current_dir.display().to_string()
});
pub(crate) fn user_dir() -> &'static str {
    &USER_DIR
}

/// The password database entry of the real user, which HotSpot reads the user properties from
#[cfg(unix)]
static PASSWD_ENTRY: LazyLock<Option<nix::unistd::User>> = LazyLock::new(|| {
    nix::unistd::User::from_uid(nix::unistd::getuid())
        .ok()
        .flatten()
});

#[cfg(unix)]
static USER_NAME: LazyLock<String> = LazyLock::new(|| match PASSWD_ENTRY.as_ref() {
    Some(entry) => entry.name.clone(),
    None => "?".to_string(),
});

#[cfg(windows)]
static USER_NAME: LazyLock<String> = LazyLock::new(|| {
    use winapi::shared::winerror::ERROR_INSUFFICIENT_BUFFER;
    use winapi::um::errhandlingapi::GetLastError;
    use winapi::um::winbase::GetUserNameW;

    // like OpenJDK, %USERNAME% first, as GetUserNameW is expensive
    if let Some(name) = env::var_os("USERNAME").filter(|name| !name.is_empty()) {
        return name.to_string_lossy().into_owned();
    }
    let mut length = 0;
    if unsafe { GetUserNameW(std::ptr::null_mut(), &mut length) } != 0
        || unsafe { GetLastError() } != ERROR_INSUFFICIENT_BUFFER
    {
        return "unknown".to_string();
    }
    let mut name = vec![0u16; length as usize];
    if unsafe { GetUserNameW(name.as_mut_ptr(), &mut length) } == 0 {
        return "unknown".to_string();
    }
    // the length counts the terminating null
    String::from_utf16_lossy(&name[..(length as usize).saturating_sub(1)])
});

pub(crate) fn user_name() -> &'static str {
    &USER_NAME
}

/// The home directory the OS reports: macOS asks Foundation, which also honors `CFFIXED_USER_HOME`
#[cfg(target_os = "macos")]
fn os_user_home() -> Option<String> {
    objc2::rc::autoreleasepool(|_| Some(objc2_foundation::NSHomeDirectory().to_string()))
}

#[cfg(all(unix, not(target_os = "macos")))]
fn os_user_home() -> Option<String> {
    PASSWD_ENTRY
        .as_ref()
        .map(|entry| entry.dir.to_string_lossy().into_owned())
}

#[cfg(unix)]
static USER_HOME: LazyLock<String> = LazyLock::new(|| {
    // like OpenJDK, $HOME only stands in for a missing home directory or one shorter than two
    // characters, such as `/`
    match os_user_home() {
        Some(home) if home.len() >= 2 => home,
        _ => env::var_os("HOME")
            .filter(|home| !home.is_empty())
            .map_or_else(
                || "?".to_string(),
                |home| home.to_string_lossy().into_owned(),
            ),
    }
});

#[cfg(windows)]
static USER_HOME: LazyLock<String> = LazyLock::new(|| {
    use winapi::um::combaseapi::CoTaskMemFree;
    use winapi::um::knownfolders::FOLDERID_Profile;
    use winapi::um::shlobj::{SHGetKnownFolderPath, KF_FLAG_DONT_VERIFY};

    let mut path = std::ptr::null_mut();
    let result = unsafe {
        SHGetKnownFolderPath(
            &FOLDERID_Profile,
            KF_FLAG_DONT_VERIFY,
            std::ptr::null_mut(),
            &mut path,
        )
    };
    let home = (result >= 0).then(|| {
        let length = (0..).take_while(|&i| unsafe { *path.add(i) } != 0).count();
        String::from_utf16_lossy(unsafe { std::slice::from_raw_parts(path, length) })
    });
    unsafe { CoTaskMemFree(path.cast()) };
    home.unwrap_or_else(|| "C:\\".to_string())
});

pub(crate) fn user_home() -> &'static str {
    &USER_HOME
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
