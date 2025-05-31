use crate::vm::error::{Error, Result};
use crate::vm::system_native::properties_provider::properties::{
    endianness, file_separator, line_separator, os_name, os_version, path_separator, user_dir,
};
use indexmap::IndexMap;
use once_cell::sync::OnceCell;
use std::sync::LazyLock;

static DEFAULT_PLATFORM_PROPERTIES: LazyLock<IndexMap<&str, &str>> = LazyLock::new(|| {
    IndexMap::from([
        ("display.country", "display.country_VALUE"),
        ("display.language", "display.language_VALUE"),
        ("display.script", "display.script_VALUE"),
        ("display.variant", "display.variant_VALUE"),
        ("file.encoding", "file.encoding_VALUE"),
        ("file.separator", file_separator()),
        ("format.country", "format.country_VALUE"),
        ("format.language", "format.language_VALUE"),
        ("format.script", "format.script_VALUE"),
        ("format.variant", "format.variant_VALUE"),
        ("ftp.nonProxyHosts", "ftp.nonProxyHosts_VALUE"),
        ("ftp.proxyHost", "ftp.proxyHost_VALUE"),
        ("ftp.proxyPort", "ftp.proxyPort_VALUE"),
        ("http.nonProxyHosts", "http.nonProxyHosts_VALUE"),
        ("http.proxyHost", "http.proxyHost_VALUE"),
        ("http.proxyPort", "http.proxyPort_VALUE"),
        ("https.proxyHost", "https.proxyHost_VALUE"),
        ("https.proxyPort", "https.proxyPort_VALUE"),
        ("java.io.tmpdir", "java.io.tmpdir_VALUE"),
        ("line.separator", line_separator()),
        ("os.arch", "os.arch_VALUE"),
        ("os.name", os_name()),
        ("os.version", os_version()),
        ("path.separator", path_separator()),
        ("socksNonProxyHosts", "socksNonProxyHosts_VALUE"),
        ("socksProxyHost", "socksProxyHost_VALUE"),
        ("socksProxyPort", "socksProxyPort_VALUE"),
        ("stderr.encoding", "stderr.encoding_VALUE"),
        ("stdout.encoding", "stdout.encoding_VALUE"),
        ("sun.arch.abi", "sun.arch.abi_VALUE"),
        ("sun.arch.data.model", "sun.arch.data.model_VALUE"),
        ("sun.cpu.endian", endianness()),
        ("sun.cpu.isalist", "sun.cpu.isalist_VALUE"),
        ("sun.io.unicode.encoding", "sun.io.unicode.encoding_VALUE"),
        ("sun.jnu.encoding", "sun.jnu.encoding_VALUE"),
        ("sun.os.patch.level", "sun.os.patch.level_VALUE"),
        ("user.dir", user_dir()),
        ("user.home", "user.home_VALUE"),
        ("user.name", "user.name_VALUE"),
    ])
});

static DEFAULT_VM_PROPERTIES: LazyLock<IndexMap<&str, &str>> =
    LazyLock::new(|| IndexMap::from([("java.home", "java.home_DEFAULT")]));

pub static OVERRIDDEN_PLATFORM_PROPERTIES: OnceCell<IndexMap<String, String>> = OnceCell::new();
pub static OVERRIDDEN_VM_PROPERTIES: OnceCell<IndexMap<String, String>> = OnceCell::new();

pub fn init_system_properties(system_properties: IndexMap<String, String>) -> Result<()> {
    let mut overridden_platform_props = deep_clone(&DEFAULT_PLATFORM_PROPERTIES);
    let mut overridden_vm_props = deep_clone(&DEFAULT_VM_PROPERTIES);

    let (platform, vm): (IndexMap<String, String>, IndexMap<String, String>) = system_properties
        .into_iter()
        .partition(|(key, _value)| overridden_platform_props.contains_key(key));

    overridden_platform_props.extend(platform);
    overridden_vm_props.extend(vm);

    OVERRIDDEN_PLATFORM_PROPERTIES
        .set(overridden_platform_props)
        .map_err(|existing_value| {
            Error::new_execution(&format!(
                "OVERRIDDEN_PLATFORM_PROPERTIES already initialized: {existing_value:?}"
            ))
        })?;
    OVERRIDDEN_VM_PROPERTIES
        .set(overridden_vm_props)
        .map_err(|existing_value| {
            Error::new_execution(&format!(
                "OVERRIDDEN_VM_PROPERTIES already initialized: {existing_value:?}"
            ))
        })
}

fn deep_clone(to_clone: &IndexMap<&str, &str>) -> IndexMap<String, String> {
    to_clone
        .clone()
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_system_properties() {
        let system_properties = IndexMap::from([
            ("os.name".to_string(), "new_os_name".to_string()),
            ("java.home".to_string(), "new_java_home".to_string()),
            ("other.property".to_string(), "other_value".to_string()),
        ]);

        assert!(init_system_properties(system_properties).is_ok());
        assert!(OVERRIDDEN_VM_PROPERTIES.get().is_some());
        assert!(OVERRIDDEN_PLATFORM_PROPERTIES.get().is_some());

        let platform_properties = OVERRIDDEN_PLATFORM_PROPERTIES.get().unwrap();
        let vm_properties = OVERRIDDEN_VM_PROPERTIES.get().unwrap();
        assert_eq!(
            platform_properties.get("os.name"),
            Some(&"new_os_name".to_string())
        );
        assert_eq!(platform_properties.len(), 39);

        assert_eq!(
            vm_properties,
            &IndexMap::from([
                ("java.home".to_string(), "new_java_home".to_string()),
                ("other.property".to_string(), "other_value".to_string())
            ])
        );
    }
}
