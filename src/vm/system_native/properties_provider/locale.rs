//! The host locales from which `SystemProps` sets `user.language`, `user.script`, `user.country`
//! and `user.variant`, and their `.display` and `.format` forms, found as HotSpot does in
//! `java_props_md.c` and, on macOS, `java_props_macosx.c`.

use std::sync::LazyLock;

/// A host locale in the parts HotSpot passes to `SystemProps`. A part that is `None` leaves its
/// property unset.
#[derive(Debug, Default, PartialEq)]
pub(crate) struct HostLocale {
    pub(crate) language: Option<String>,
    pub(crate) script: Option<String>,
    pub(crate) country: Option<String>,
    pub(crate) variant: Option<String>,
}

/// The locale for formatting values and the one for the user interface.
struct HostLocales {
    format: HostLocale,
    display: HostLocale,
}

static HOST_LOCALES: LazyLock<HostLocales> = LazyLock::new(host_locales);

/// The locale for formatting values, `format.*` in `SystemProps`.
pub(crate) fn format_locale() -> &'static HostLocale {
    &HOST_LOCALES.format
}

/// The locale for the user interface, `display.*` in `SystemProps`.
pub(crate) fn display_locale() -> &'static HostLocale {
    &HOST_LOCALES.display
}

#[cfg(unix)]
fn host_locales() -> HostLocales {
    use nix::libc::{setlocale, LC_ALL, LC_CTYPE, LC_MESSAGES};

    unsafe { setlocale(LC_ALL, c"".as_ptr()) };
    match parse_locale(category_locale(LC_CTYPE)) {
        Some(format) => HostLocales {
            format,
            display: parse_locale(category_locale(LC_MESSAGES)).unwrap_or_default(),
        },
        None => HostLocales {
            format: HostLocale::default(),
            display: HostLocale {
                language: Some("en".to_string()),
                ..HostLocale::default()
            },
        },
    }
}

/// The name of the locale of a category, as `setlocale` gives it.
#[cfg(unix)]
fn posix_locale(category: nix::libc::c_int) -> Option<String> {
    let name = unsafe { nix::libc::setlocale(category, std::ptr::null()) };
    if name.is_null() {
        return None;
    }
    let name = unsafe { std::ffi::CStr::from_ptr(name) };
    Some(name.to_string_lossy().into_owned())
}

#[cfg(target_os = "linux")]
fn category_locale(category: nix::libc::c_int) -> Option<String> {
    posix_locale(category)
}

/// Like HotSpot's `setupMacOSXLocale`: the preferred language, or else the POSIX locale or `LANG`.
#[cfg(target_os = "macos")]
fn category_locale(category: nix::libc::c_int) -> Option<String> {
    preferred_language().or_else(|| match posix_locale(category) {
        None => std::env::var_os("LANG").map(|lang| lang.to_string_lossy().into_owned()),
        Some(name) if name == "C" => {
            std::env::var_os("LANG").map(|lang| lang.to_string_lossy().into_owned())
        }
        name => name,
    })
}

/// Like HotSpot's `getMacOSXLocale`: the first preferred language, which since macOS 10.12 is used
/// for all categories, with the region of the current locale if it has none, as a POSIX locale
/// name.
#[cfg(target_os = "macos")]
fn preferred_language() -> Option<String> {
    use objc2_foundation::NSLocale;

    let mut language = NSLocale::preferredLanguages().firstObject()?.to_string();
    let script_or_region = language.find('-').map(|hyphen| &language[hyphen..]);
    // "en", or "en-Latn" with a script only
    if script_or_region.map_or(true, |rest| rest.len() == 5) {
        let current = NSLocale::currentLocale().localeIdentifier().to_string();
        if let Some(underscore) = current.rfind('_') {
            language.push('-');
            language.push_str(&current[underscore + 1..]);
        }
    }
    // the conversion doesn't expect variants
    if let Some(at) = language.find('@') {
        language.truncate(at);
    }
    Some(convert_to_posix_locale(&language))
}

/// Like HotSpot's `convertToPOSIXLocale`: a BCP 47 language ID, which is a language with an
/// optional script and region, as a POSIX locale name `language_region@script`. For example,
/// `zh-Hans-US` becomes `zh_US@Hans`, `es-419` becomes `es_419`, and `zh-Hans` becomes `zh@Hans`.
#[cfg(target_os = "macos")]
fn convert_to_posix_locale(id: &str) -> String {
    let Some((language, rest)) = id.split_once('-') else {
        return id.to_string();
    };
    match (rest.get(..4), rest.get(5..)) {
        (Some(script), Some(region)) if rest.len() > 4 => format!("{language}_{region}@{script}"),
        _ if rest.len() == 4 => format!("{language}@{rest}"),
        _ => format!("{language}_{rest}"),
    }
}

/// Like HotSpot's `ParseLocale`: the parts of a POSIX locale name, `language_country.encoding@modifier`,
/// mapped to Java's. `None` if there is no locale name, except on Linux, where that, `C` and
/// `POSIX` stand for `en_US`.
#[cfg(unix)]
fn parse_locale(name: Option<String>) -> Option<HostLocale> {
    #[cfg(target_os = "linux")]
    let name = match name.as_deref() {
        None | Some("C") | Some("POSIX") => "en_US".to_string(),
        Some(_) => name.unwrap_or_default(),
    };
    #[cfg(not(target_os = "linux"))]
    let name = name?;

    let (mut base, mut encoding_modifier) = split_encoding_modifier(&name);
    if let Some(alias) = lookup(LOCALE_ALIASES, LOCALE_ALIASES_PLATFORM, base) {
        let (alias_base, alias_encoding_modifier) = split_encoding_modifier(alias);
        base = alias_base;
        // HotSpot keeps the encoding and modifier of the name unless the alias has its own
        if !alias_encoding_modifier.is_empty() {
            encoding_modifier = alias_encoding_modifier;
        }
    }

    let (language, country) = match base.split_once('_') {
        Some((language, country)) => (language, Some(country)),
        None => (base, None),
    };
    let after_encoding = match encoding_modifier.split_once('.') {
        Some((_, after)) => after,
        None => encoding_modifier,
    };
    let modifier = after_encoding.split_once('@').map(|(_, modifier)| modifier);

    let language = lookup(LANGUAGE_NAMES, LANGUAGE_NAMES_PLATFORM, language).unwrap_or(language);
    let country = country
        .map(|country| lookup(COUNTRY_NAMES, COUNTRY_NAMES_PLATFORM, country).unwrap_or(country));
    let script =
        modifier.and_then(|modifier| lookup(SCRIPT_NAMES, SCRIPT_NAMES_PLATFORM, modifier));
    let variant =
        modifier.and_then(|modifier| lookup(VARIANT_NAMES, VARIANT_NAMES_PLATFORM, modifier));
    Some(HostLocale {
        language: Some(language.to_string()),
        script: script.map(str::to_string),
        country: country.map(str::to_string),
        variant: variant.map(str::to_string),
    })
}

/// Splits a locale name before its encoding (`.`), or else before its modifier (`@`).
#[cfg(unix)]
fn split_encoding_modifier(name: &str) -> (&str, &str) {
    match name.find('.').or_else(|| name.find('@')) {
        Some(index) => name.split_at(index),
        None => (name, ""),
    }
}

#[cfg(unix)]
fn lookup(
    table: &[(&str, &'static str)],
    platform_table: &[(&str, &'static str)],
    key: &str,
) -> Option<&'static str> {
    table
        .iter()
        .chain(platform_table)
        .find(|(name, _)| *name == key)
        .map(|(_, value)| *value)
}

#[cfg(windows)]
fn host_locales() -> HostLocales {
    use winapi::um::winnls::{GetUserDefaultLCID, GetUserDefaultUILanguage};
    use winapi::um::winnt::{LANGIDFROMLCID, MAKELCID, PRIMARYLANGID, SORTIDFROMLCID};

    let user_lcid = unsafe { GetUserDefaultLCID() };
    let ui_language = unsafe { GetUserDefaultUILanguage() };
    // Like HotSpot, use the region of the user locale for the UI language if they have the same
    // language: Windows lets users choose only the language of the UI, e.g. "English", not en_GB
    let ui_lcid = if PRIMARYLANGID(LANGIDFROMLCID(user_lcid)) == PRIMARYLANGID(ui_language) {
        user_lcid
    } else {
        MAKELCID(ui_language, SORTIDFROMLCID(user_lcid))
    };
    HostLocales {
        format: windows_locale(user_lcid),
        display: windows_locale(ui_lcid),
    }
}

/// Like HotSpot's `SetupI18nProps`. All parts are set, if only to an empty string.
#[cfg(windows)]
fn windows_locale(lcid: winapi::um::winnt::LCID) -> HostLocale {
    use winapi::um::winnls::LCTYPE;
    // WinNls.h, which winapi lacks
    const LOCALE_SNAME: LCTYPE = 0x5c;
    const LOCALE_SISO639LANGNAME: LCTYPE = 0x59;
    const LOCALE_SISO3166CTRYNAME: LCTYPE = 0x5a;
    const LOCALE_SISO639LANGNAME2: LCTYPE = 0x67;
    const LOCALE_SISO3166CTRYNAME2: LCTYPE = 0x68;
    // HotSpot's buffer sizes: a longer value fails
    const PROPSIZE: usize = 9;
    const SNAMESIZE: usize = 86;

    let info = |info_type, size| locale_info(lcid, info_type, size);
    let script = info(LOCALE_SNAME, SNAMESIZE)
        .and_then(|name| script_of(&name))
        .unwrap_or_default();
    let mut country = info(LOCALE_SISO3166CTRYNAME, PROPSIZE)
        .or_else(|| info(LOCALE_SISO3166CTRYNAME2, PROPSIZE))
        .unwrap_or_default();
    let mut language = info(LOCALE_SISO639LANGNAME, PROPSIZE)
        .or_else(|| info(LOCALE_SISO639LANGNAME2, PROPSIZE))
        .unwrap_or_else(|| {
            country = "US".to_string();
            "en".to_string()
        });
    let mut variant = String::new();
    match language.as_str() {
        "nb" => {
            language = "no".to_string();
            country = "NO".to_string();
        }
        "nn" => {
            language = "no".to_string();
            country = "NO".to_string();
            variant = "NY".to_string();
        }
        _ => {}
    }

    HostLocale {
        language: Some(language),
        script: Some(script),
        country: Some(country),
        variant: Some(variant),
    }
}

/// A value of `GetLocaleInfo` that fits in `size` characters with the terminating null.
#[cfg(windows)]
fn locale_info(
    lcid: winapi::um::winnt::LCID,
    info_type: winapi::um::winnls::LCTYPE,
    size: usize,
) -> Option<String> {
    use winapi::um::winnls::GetLocaleInfoW;

    let mut buffer = vec![0u16; size];
    let len = unsafe { GetLocaleInfoW(lcid, info_type, buffer.as_mut_ptr(), size as i32) };
    // the length includes the terminating null
    let len = usize::try_from(len).ok()?.checked_sub(1)?;
    Some(String::from_utf16_lossy(&buffer[..len]))
}

/// The script of a locale name such as `sr-Latn-RS`, as HotSpot reads it with
/// `sscanf("%*[a-z\\-]%1[A-Z]%[a-z]")`: a capital and three lowercase letters after the lowercase
/// language and a hyphen.
#[cfg(windows)]
fn script_of(name: &str) -> Option<String> {
    let rest = name.trim_start_matches(|c: char| c.is_ascii_lowercase() || c == '-');
    if rest.len() == name.len() {
        return None;
    }
    let mut chars = rest.chars();
    let first = chars.next().filter(char::is_ascii_uppercase)?;
    let script: String = std::iter::once(first)
        .chain(chars.take_while(char::is_ascii_lowercase))
        .collect();
    (script.len() == 4).then_some(script)
}

// The tables of HotSpot's locale_str.h

/// Full locale names for partial ones, mostly a language without a country.
#[cfg(unix)]
const LOCALE_ALIASES: &[(&str, &str)] = &[
    ("ar", "ar_EG"),
    ("be", "be_BY"),
    ("bg", "bg_BG"),
    ("br", "br_FR"),
    ("ca", "ca_ES"),
    ("cs", "cs_CZ"),
    ("cz", "cs_CZ"),
    ("da", "da_DK"),
    ("de", "de_DE"),
    ("el", "el_GR"),
    ("en", "en_US"),
    ("eo", "eo"),
    ("es", "es_ES"),
    ("et", "et_EE"),
    ("eu", "eu_ES"),
    ("fi", "fi_FI"),
    ("fr", "fr_FR"),
    ("ga", "ga_IE"),
    ("gl", "gl_ES"),
    ("he", "iw_IL"),
    ("hr", "hr_HR"),
    ("hu", "hu_HU"),
    ("id", "in_ID"),
    ("in", "in_ID"),
    ("is", "is_IS"),
    ("it", "it_IT"),
    ("iw", "iw_IL"),
    ("ja", "ja_JP"),
    ("kl", "kl_GL"),
    ("ko", "ko_KR"),
    ("lt", "lt_LT"),
    ("lv", "lv_LV"),
    ("mk", "mk_MK"),
    ("nl", "nl_NL"),
    ("no", "no_NO"),
    ("pl", "pl_PL"),
    ("pt", "pt_PT"),
    ("ro", "ro_RO"),
    ("ru", "ru_RU"),
    ("se", "se_NO"),
    ("sk", "sk_SK"),
    ("sl", "sl_SI"),
    ("sq", "sq_AL"),
    ("sr", "sr_CS"),
    ("su", "fi_FI"),
    ("sv", "sv_SE"),
    ("th", "th_TH"),
    ("tr", "tr_TR"),
    ("uk", "uk_UA"),
    ("vi", "vi_VN"),
    ("wa", "wa_BE"),
    ("zh", "zh_CN"),
];
#[cfg(target_os = "linux")]
const LOCALE_ALIASES_PLATFORM: &[(&str, &str)] = &[
    ("hs", "en_US"),
    ("ua", "en_US"),
    ("bokmal", "nb_NO"),
    ("bokmål", "nb_NO"),
    ("catalan", "ca_ES"),
    ("croatian", "hr_HR"),
    ("czech", "cs_CZ"),
    ("danish", "da_DK"),
    ("dansk", "da_DK"),
    ("deutsch", "de_DE"),
    ("dutch", "nl_NL"),
    ("eesti", "et_EE"),
    ("estonian", "et_EE"),
    ("finnish", "fi_FI"),
    ("français", "fr_FR"),
    ("french", "fr_FR"),
    ("galego", "gl_ES"),
    ("galician", "gl_ES"),
    ("german", "de_DE"),
    ("greek", "el_GR"),
    ("hebrew", "iw_IL"),
    ("hrvatski", "hr_HR"),
    ("hungarian", "hu_HU"),
    ("icelandic", "is_IS"),
    ("italian", "it_IT"),
    ("japanese", "ja_JP"),
    ("korean", "ko_KR"),
    ("lithuanian", "lt_LT"),
    ("norwegian", "no_NO"),
    ("nynorsk", "nn_NO"),
    ("polish", "pl_PL"),
    ("portuguese", "pt_PT"),
    ("romanian", "ro_RO"),
    ("russian", "ru_RU"),
    ("slovak", "sk_SK"),
    ("slovene", "sl_SI"),
    ("slovenian", "sl_SI"),
    ("spanish", "es_ES"),
    ("swedish", "sv_SE"),
    ("thai", "th_TH"),
    ("turkish", "tr_TR"),
];
#[cfg(all(unix, not(target_os = "linux")))]
const LOCALE_ALIASES_PLATFORM: &[(&str, &str)] = &[
    ("big5", "zh_TW.Big5"),
    ("chinese", "zh_CN"),
    ("iso_8859_1", "en_US.ISO8859-1"),
    ("iso_8859_15", "en_US.ISO8859-15"),
    ("japanese", "ja_JP"),
    ("no_NY", "no_NO@nynorsk"),
    ("sr_SP", "sr_YU"),
    ("tchinese", "zh_TW"),
];

/// ISO 639 language codes for the language names of host locales.
#[cfg(unix)]
const LANGUAGE_NAMES: &[(&str, &str)] = &[
    ("C", "en"),
    ("POSIX", "en"),
    ("cz", "cs"),
    ("he", "iw"),
    ("id", "in"),
    ("sh", "sr"),
    ("su", "fi"),
];
#[cfg(target_os = "linux")]
const LANGUAGE_NAMES_PLATFORM: &[(&str, &str)] = &[
    ("hs", "en"),
    ("ua", "en"),
    ("catalan", "ca"),
    ("croatian", "hr"),
    ("czech", "cs"),
    ("danish", "da"),
    ("dansk", "da"),
    ("deutsch", "de"),
    ("dutch", "nl"),
    ("finnish", "fi"),
    ("français", "fr"),
    ("french", "fr"),
    ("german", "de"),
    ("greek", "el"),
    ("hebrew", "he"),
    ("hrvatski", "hr"),
    ("hungarian", "hu"),
    ("icelandic", "is"),
    ("italian", "it"),
    ("japanese", "ja"),
    ("norwegian", "no"),
    ("polish", "pl"),
    ("portuguese", "pt"),
    ("romanian", "ro"),
    ("russian", "ru"),
    ("slovak", "sk"),
    ("slovene", "sl"),
    ("slovenian", "sl"),
    ("spanish", "es"),
    ("swedish", "sv"),
    ("turkish", "tr"),
];
#[cfg(all(unix, not(target_os = "linux")))]
const LANGUAGE_NAMES_PLATFORM: &[(&str, &str)] =
    &[("chinese", "zh"), ("japanese", "ja"), ("korean", "ko")];

/// Script codes for the modifiers (`@...`) of host locales.
#[cfg(unix)]
const SCRIPT_NAMES: &[(&str, &str)] = &[
    ("Arab", "Arab"),
    ("Cyrl", "Cyrl"),
    ("Deva", "Deva"),
    ("Ethi", "Ethi"),
    ("Hans", "Hans"),
    ("Hant", "Hant"),
    ("Latn", "Latn"),
    ("Sund", "Sund"),
    ("Syrc", "Syrc"),
    ("Tfng", "Tfng"),
];
#[cfg(target_os = "linux")]
const SCRIPT_NAMES_PLATFORM: &[(&str, &str)] = &[
    ("cyrillic", "Cyrl"),
    ("devanagari", "Deva"),
    ("iqtelif", "Latn"),
    ("latin", "Latn"),
];
#[cfg(all(unix, not(target_os = "linux")))]
const SCRIPT_NAMES_PLATFORM: &[(&str, &str)] = &[];

/// ISO 3166 country codes for the country names of host locales.
#[cfg(unix)]
const COUNTRY_NAMES: &[(&str, &str)] = &[("YU", "CS")];
#[cfg(target_os = "linux")]
const COUNTRY_NAMES_PLATFORM: &[(&str, &str)] = &[("RN", "US")];
#[cfg(all(unix, not(target_os = "linux")))]
const COUNTRY_NAMES_PLATFORM: &[(&str, &str)] = &[];

/// Java variants for the modifiers (`@...`) of host locales; other modifiers are ignored.
#[cfg(unix)]
const VARIANT_NAMES: &[(&str, &str)] = &[("nynorsk", "NY")];
#[cfg(target_os = "linux")]
const VARIANT_NAMES_PLATFORM: &[(&str, &str)] = &[];
#[cfg(all(unix, not(target_os = "linux")))]
const VARIANT_NAMES_PLATFORM: &[(&str, &str)] = &[];

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    fn parsed(name: Option<&str>) -> Option<HostLocale> {
        parse_locale(name.map(str::to_string))
    }

    #[cfg(unix)]
    fn expected(
        language: &str,
        script: Option<&str>,
        country: Option<&str>,
        variant: Option<&str>,
    ) -> Option<HostLocale> {
        Some(HostLocale {
            language: Some(language.to_string()),
            script: script.map(str::to_string),
            country: country.map(str::to_string),
            variant: variant.map(str::to_string),
        })
    }

    #[cfg(unix)]
    #[test]
    fn should_parse_locale_names_like_hotspot() {
        let us = expected("en", None, Some("US"), None);
        assert_eq!(parsed(Some("en_US.UTF-8")), us);
        assert_eq!(
            parsed(Some("de_DE.UTF-8")),
            expected("de", None, Some("DE"), None)
        );
        assert_eq!(
            parsed(Some("de_DE@euro")),
            expected("de", None, Some("DE"), None)
        );
        // aliases add a country, and keep the encoding of the name
        assert_eq!(parsed(Some("en")), us);
        assert_eq!(
            parsed(Some("ja.UTF-8")),
            expected("ja", None, Some("JP"), None)
        );
        assert_eq!(parsed(Some("eo")), expected("eo", None, None, None));
        // old ISO 639 codes, as in Locale
        assert_eq!(
            parsed(Some("he_IL.UTF-8")),
            expected("iw", None, Some("IL"), None)
        );
        assert_eq!(
            parsed(Some("id_ID")),
            expected("in", None, Some("ID"), None)
        );
        assert_eq!(
            parsed(Some("sr_YU")),
            expected("sr", None, Some("CS"), None)
        );
        assert_eq!(parsed(Some("C.UTF-8")), expected("en", None, None, None));
        assert_eq!(
            parsed(Some("no_NO@nynorsk")),
            expected("no", None, Some("NO"), Some("NY"))
        );
        assert_eq!(
            parsed(Some("zh_TW@Hant")),
            expected("zh", Some("Hant"), Some("TW"), None)
        );
        assert_eq!(parsed(Some("en_US@unknown")), us);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn should_parse_linux_locale_names_like_hotspot() {
        let us = expected("en", None, Some("US"), None);
        assert_eq!(parsed(None), us);
        assert_eq!(parsed(Some("C")), us);
        assert_eq!(parsed(Some("POSIX")), us);
        assert_eq!(
            parsed(Some("sr_RS@latin")),
            expected("sr", Some("Latn"), Some("RS"), None)
        );
        assert_eq!(
            parsed(Some("german")),
            expected("de", None, Some("DE"), None)
        );
        assert_eq!(parsed(Some("en_RN")), us);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn should_parse_macos_locale_names_like_hotspot() {
        assert_eq!(parsed(None), None);
        assert_eq!(parsed(Some("C")), expected("en", None, None, None));
        assert_eq!(
            parsed(Some("sr_RS@latin")),
            expected("sr", None, Some("RS"), None)
        );
        assert_eq!(
            parsed(Some("no_NY")),
            expected("no", None, Some("NO"), Some("NY"))
        );
        assert_eq!(
            parsed(Some("zh_US@Hans")),
            expected("zh", Some("Hans"), Some("US"), None)
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn should_convert_language_ids_like_hotspot() {
        assert_eq!(convert_to_posix_locale("en"), "en");
        assert_eq!(convert_to_posix_locale("en_US"), "en_US");
        assert_eq!(convert_to_posix_locale("en-GB"), "en_GB");
        assert_eq!(convert_to_posix_locale("es-419"), "es_419");
        assert_eq!(convert_to_posix_locale("zh-Hans"), "zh@Hans");
        assert_eq!(convert_to_posix_locale("zh-Hans-US"), "zh_US@Hans");
        assert_eq!(convert_to_posix_locale("zh-Hans-419"), "zh_419@Hans");
    }

    #[cfg(windows)]
    #[test]
    fn should_read_scripts_of_locale_names_like_hotspot() {
        assert_eq!(script_of("sr-Latn-RS").as_deref(), Some("Latn"));
        assert_eq!(script_of("zh-Hans-CN").as_deref(), Some("Hans"));
        assert_eq!(script_of("en-US"), None);
        assert_eq!(script_of("en"), None);
        assert_eq!(script_of("Latn"), None);
    }
}
