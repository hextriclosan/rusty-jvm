use crate::helper::create_array_of_strings;
use crate::system_native::properties_provider::properties::{
    endianness, file_separator, line_separator, os_name, os_version, path_separator, user_dir,
};

pub(crate) fn platform_properties_wrp(_args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let string_array_ref = platform_properties()?;

    Ok(vec![string_array_ref])
}
fn platform_properties() -> crate::error::Result<i32> {
    create_array_of_strings(get_platform_properties().as_slice())
}

pub(crate) fn vm_properties_wrp(_args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let string_array_ref = vm_properties()?;

    Ok(vec![string_array_ref])
}
fn vm_properties() -> crate::error::Result<i32> {
    let x = get_vm_properties();
    let x_refs: Vec<&str> = x.iter().map(|s| s.as_str()).collect();
    create_array_of_strings(&x_refs)
}

fn get_platform_properties() -> Vec<&'static str> {
    vec![
        "display.country_VALUE",
        "display.language_VALUE",
        "display.script_VALUE",
        "display.variant_VALUE",
        "file.encoding_VALUE",
        file_separator(), // "file.separator"
        "format.country_VALUE",
        "format.language_VALUE",
        "format.script_VALUE",
        "format.variant_VALUE",
        "ftp.nonProxyHosts_VALUE",
        "ftp.proxyHost_VALUE",
        "ftp.proxyPort_VALUE",
        "http.nonProxyHosts_VALUE",
        "http.proxyHost_VALUE",
        "http.proxyPort_VALUE",
        "https.proxyHost_VALUE",
        "https.proxyPort_VALUE",
        "java.io.tmpdir_VALUE",
        line_separator(), // "line.separator"
        "os.arch_VALUE",
        os_name(),        // "os.name"
        os_version(),     // "os.version"
        path_separator(), // "path.separator"
        "socksNonProxyHosts_VALUE",
        "socksProxyHost_VALUE",
        "socksProxyPort_VALUE",
        "stderr.encoding_VALUE",
        "stdout.encoding_VALUE",
        "sun.arch.abi_VALUE",
        "sun.arch.data.model_VALUE",
        endianness(), // "sun.cpu.endian"
        "sun.cpu.isalist_VALUE",
        "sun.io.unicode.encoding_VALUE",
        "sun.jnu.encoding_VALUE",
        "sun.os.patch.level_VALUE",
        user_dir(), // "user.dir"
        "user.home_VALUE",
        "user.name_VALUE",
    ]
}

fn get_vm_properties() -> Vec<String> {
    let mut out = vec![
        "java.home".to_string(),
        "java.home_VALUE".to_string(),
        //"jdk.invoke.MethodHandle.dumpClassFiles".to_string(), "true".to_string()
    ];
    //let x = SYSTEM_PROPERTIES.get().expect("Failed to get system properties");
    //out.extend_from_slice(x.as_slice());

    out
}
