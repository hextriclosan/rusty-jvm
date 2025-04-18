use crate::execution_engine::string_pool_helper::StringPoolHelper;
use crate::heap::heap::with_heap_write_lock;
use crate::system_native::properties_provider::properties::{
    endianness, file_separator, line_separator, os_name, os_version, path_separator, user_dir,
};

pub(crate) fn platform_properties_wrp(_args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let string_array_ref = platform_properties()?;

    Ok(vec![string_array_ref])
}
fn platform_properties() -> crate::error::Result<i32> {
    create_empty_array_of_strings(get_platform_properties().as_slice())
}

pub(crate) fn vm_properties_wrp(_args: &[i32]) -> crate::error::Result<Vec<i32>> {
    let string_array_ref = vm_properties()?;

    Ok(vec![string_array_ref])
}
fn vm_properties() -> crate::error::Result<i32> {
    create_empty_array_of_strings(get_vm_properties().as_slice())
}

fn create_empty_array_of_strings(props: &[&str]) -> crate::error::Result<i32> {
    let class_of_array = "java/lang/String";
    let class_of_array = format!("[L{class_of_array};");
    let length = props.len() as i32;
    let array_ref = with_heap_write_lock(|heap| heap.create_array(&class_of_array, length))?;

    for (index, prop) in props.iter().enumerate() {
        let string_ref = StringPoolHelper::get_string(prop.to_string())?;
        with_heap_write_lock(|heap| {
            heap.set_array_value(array_ref, index as i32, vec![string_ref])
        })?
    }

    Ok(array_ref)
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

fn get_vm_properties() -> Vec<&'static str> {
    vec!["java.home", "java.home_VALUE"]
}
