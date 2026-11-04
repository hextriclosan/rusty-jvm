//! Parsing and installation of the VM's managed-heap and collection options.
//!
//! The configured maximum bounds policy-v1 managed storage accounting, not process RSS.

use crate::vm::error::{Error, Result};
use crate::Arguments;
use std::sync::OnceLock;

const DEFAULT_MAX_HEAP_BYTES: u64 = 1024 * 1024 * 1024;
const DEFAULT_GC_ALLOC_THRESHOLD: u64 = 65_536;

static CONFIG: OnceLock<GcConfig> = OnceLock::new();

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct GcConfig {
    pub(crate) max_heap_bytes: u64,
    pub(crate) gc_alloc_threshold: u64,
    pub(crate) fixed_threshold: bool,
    pub(crate) log_gc: bool,
    pub(crate) stress_at_safepoints: bool,
}

impl GcConfig {
    fn parse(
        jvm_options: &[String],
        advanced_jvm_options: &[String],
    ) -> std::result::Result<Self, String> {
        let mut max_heap_bytes = None;
        let mut gc_alloc_threshold = None;
        let mut log_gc = false;
        let mut stress_at_safepoints = false;

        for option in jvm_options {
            if let Some(value) = option.strip_prefix("-Xmx") {
                max_heap_bytes = Some(
                    parse_heap_size(value)
                        .map_err(|error| format!("Invalid VM option `{option}`: {error}"))?,
                );
            } else if option == "-Xlog:gc" {
                log_gc = true;
            }
        }

        for option in advanced_jvm_options {
            if let Some(value) = option.strip_prefix("-XX:GCAllocThreshold=") {
                gc_alloc_threshold = Some(
                    parse_positive_integer(value, "-XX:GCAllocThreshold")
                        .map_err(|error| format!("Invalid VM option `{option}`: {error}"))?,
                );
            } else if option == "-XX:+GcStressAtSafepoints" {
                stress_at_safepoints = true;
            }
        }

        Ok(Self {
            max_heap_bytes: max_heap_bytes.unwrap_or_else(default_max_heap_bytes),
            gc_alloc_threshold: gc_alloc_threshold.unwrap_or(DEFAULT_GC_ALLOC_THRESHOLD),
            fixed_threshold: gc_alloc_threshold.is_some(),
            log_gc,
            stress_at_safepoints,
        })
    }
}

pub(crate) fn install(arguments: &Arguments) -> Result<()> {
    let config = GcConfig::parse(arguments.jvm_options(), arguments.advanced_jvm_options())
        .map_err(|error| Error::new_execution(&error))?;
    CONFIG
        .set(config)
        .map_err(|_| Error::new_execution("GC configuration already installed"))?;
    Ok(())
}

pub(crate) fn get() -> Option<&'static GcConfig> {
    CONFIG.get()
}

pub(crate) fn logging_enabled() -> bool {
    get().is_some_and(|config| config.log_gc)
}

fn parse_heap_size(value: &str) -> std::result::Result<u64, String> {
    let (digits, multiplier) = match value.as_bytes().last().copied() {
        Some(b'k' | b'K') => (&value[..value.len() - 1], 1024),
        Some(b'm' | b'M') => (&value[..value.len() - 1], 1024_u64.pow(2)),
        Some(b'g' | b'G') => (&value[..value.len() - 1], 1024_u64.pow(3)),
        _ => (value, 1),
    };
    let value = parse_positive_integer(digits, "-Xmx")?;
    value
        .checked_mul(multiplier)
        .filter(|bytes| *bytes > 0)
        .ok_or_else(|| "Invalid -Xmx value: size overflows u64".to_string())
}

fn parse_positive_integer(value: &str, option: &str) -> std::result::Result<u64, String> {
    let parsed = value
        .parse::<u64>()
        .map_err(|_| format!("Invalid {option} value `{value}`: expected a positive integer"))?;
    if parsed == 0 {
        return Err(format!(
            "Invalid {option} value `{value}`: value must be positive"
        ));
    }
    Ok(parsed)
}

fn default_max_heap_bytes() -> u64 {
    #[cfg(target_os = "linux")]
    {
        let meminfo = std::fs::read_to_string("/proc/meminfo").ok();
        default_max_heap_bytes_from(meminfo.as_deref())
    }

    #[cfg(not(target_os = "linux"))]
    DEFAULT_MAX_HEAP_BYTES
}

#[cfg(target_os = "linux")]
fn default_max_heap_bytes_from(meminfo: Option<&str>) -> u64 {
    meminfo
        .and_then(meminfo_total_bytes)
        .unwrap_or(DEFAULT_MAX_HEAP_BYTES)
}

#[cfg(target_os = "linux")]
fn meminfo_total_bytes(meminfo: &str) -> Option<u64> {
    let value = meminfo
        .lines()
        .find_map(|line| line.strip_prefix("MemTotal:"))?
        .split_whitespace()
        .next()?
        .parse::<u64>()
        .ok()?;
    value.checked_mul(1024).map(|bytes| bytes / 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(
        jvm_options: &[&str],
        advanced_jvm_options: &[&str],
    ) -> std::result::Result<GcConfig, String> {
        GcConfig::parse(
            &jvm_options
                .iter()
                .map(|option| (*option).to_string())
                .collect::<Vec<_>>(),
            &advanced_jvm_options
                .iter()
                .map(|option| (*option).to_string())
                .collect::<Vec<_>>(),
        )
    }

    #[test]
    fn should_parse_heap_sizes_with_supported_suffixes() {
        assert_eq!(parse_heap_size("12").unwrap(), 12);
        assert_eq!(parse_heap_size("2k").unwrap(), 2 * 1024);
        assert_eq!(parse_heap_size("2K").unwrap(), 2 * 1024);
        assert_eq!(parse_heap_size("3m").unwrap(), 3 * 1024 * 1024);
        assert_eq!(parse_heap_size("3M").unwrap(), 3 * 1024 * 1024);
        assert_eq!(parse_heap_size("4g").unwrap(), 4 * 1024 * 1024 * 1024);
        assert_eq!(parse_heap_size("4G").unwrap(), 4 * 1024 * 1024 * 1024);
    }

    #[test]
    fn should_reject_invalid_heap_sizes() {
        for value in ["0", "", "12q", "999999999999999999999999999999g"] {
            assert!(parse_heap_size(value).is_err(), "accepted {value:?}");
        }
        assert!(parse_heap_size("18446744073709551615k").is_err());
    }

    #[test]
    fn should_use_default_configuration_without_gc_options() {
        let config = parse(&[], &[]).unwrap();
        assert_eq!(config.gc_alloc_threshold, DEFAULT_GC_ALLOC_THRESHOLD);
        assert!(!config.log_gc);
        assert!(!config.stress_at_safepoints);
        assert_eq!(config.max_heap_bytes, default_max_heap_bytes());
    }

    #[test]
    fn should_parse_gc_options_and_ignore_unowned_options() {
        let config = parse(
            &["-Xmx64m", "-Xlog:gc", "-Xms32m"],
            &[
                "-XX:GCAllocThreshold=128",
                "-XX:+GcStressAtSafepoints",
                "-XX:+UseG1GC",
            ],
        )
        .unwrap();
        assert_eq!(config.max_heap_bytes, 64 * 1024 * 1024);
        assert_eq!(config.gc_alloc_threshold, 128);
        assert!(config.log_gc);
        assert!(config.stress_at_safepoints);
    }

    #[test]
    fn should_reject_invalid_allocation_thresholds() {
        for value in ["0", "", "abc", "-1"] {
            let option = format!("-XX:GCAllocThreshold={value}");
            assert!(parse(&[], &[&option]).is_err(), "accepted {option:?}");
        }
    }

    #[test]
    fn should_fall_back_to_one_gibibyte_for_missing_memory_data() {
        assert_eq!(DEFAULT_MAX_HEAP_BYTES, 1_073_741_824);
        #[cfg(target_os = "linux")]
        {
            assert_eq!(default_max_heap_bytes_from(None), DEFAULT_MAX_HEAP_BYTES);
            assert_eq!(
                default_max_heap_bytes_from(Some("garbled")),
                DEFAULT_MAX_HEAP_BYTES
            );
            assert_eq!(meminfo_total_bytes("garbled"), None);
            assert_eq!(meminfo_total_bytes("MemTotal: 2048 kB\n"), Some(512 * 1024));
        }
    }
}
