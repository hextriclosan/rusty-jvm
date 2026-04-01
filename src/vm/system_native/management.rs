use crate::vm::error::Result;
use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::{create_array_of_strings, i64_to_vec};
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::stack::stack_value::StackValue;
use crate::vm::system_native::thread::current_thread_wrp;
use once_cell::sync::Lazy;
use std::time::SystemTime;

/// Wall-clock time (ms since Unix epoch) captured once when this cell is first
/// accessed.  All uptime / startup-time computations are relative to this value.
static STARTUP_TIME_MS: Lazy<u64> = Lazy::new(|| {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
});

/// Stub CPU count returned by all processor-count native methods.
/// Matches the value already returned by `java/lang/Runtime:availableProcessors`.
const STUB_CPU_COUNT: i32 = 14;

/// Two-element `Vec<i32>` encoding the IEEE 754 double `0.0`.
/// Used as the return value for all stub CPU-load native methods.
fn zero_double() -> Vec<i32> {
    0.0f64.to_vec()
}

// ─── VMManagementImpl ────────────────────────────────────────────────────────

/// Returns the JMM (Java Management Monitor) version string.
/// The JDK static initialiser splits it on "." and requires major == 1 or 2.
pub(crate) fn vm_mgmt_get_version0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    let s = crate::vm::execution_engine::string_pool_helper::StringPoolHelper::get_string("2.0")?;
    Ok(vec![s])
}

/// Sets the optional-support boolean fields in `VMManagementImpl`.
/// Rusty-JVM does not support thread contention monitoring, CPU-time
/// tracking, GC notification, etc., so all flags are left at their
/// default `false`.
pub(crate) fn vm_mgmt_init_optional_support_fields_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    const CLASS: &str = "sun/management/VMManagementImpl";
    let fields = [
        "compTimeMonitoringSupport",
        "threadContentionMonitoringSupport",
        "currentThreadCpuTimeSupport",
        "otherThreadCpuTimeSupport",
        "objectMonitorUsageSupport",
        "synchronizerUsageSupport",
        "threadAllocatedMemorySupport",
        "gcNotificationSupport",
        "remoteDiagnosticCommandsSupport",
    ];
    for field_name in fields {
        let (_, field) = with_method_area(|ma| ma.lookup_for_static_field(CLASS, field_name))?;
        field.set_raw_value(vec![0])?; // false
    }
    Ok(vec![])
}

pub(crate) fn vm_mgmt_get_startup_time_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(*STARTUP_TIME_MS as i64))
}

pub(crate) fn vm_mgmt_get_uptime0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    let now_ms = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);
    let uptime = now_ms.saturating_sub(*STARTUP_TIME_MS) as i64;
    Ok(i64_to_vec(uptime))
}

pub(crate) fn vm_mgmt_get_process_id_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![std::process::id() as i32])
}

pub(crate) fn vm_mgmt_get_vm_arguments0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    let array_ref = create_array_of_strings(&[] as &[String])?;
    Ok(vec![array_ref])
}

pub(crate) fn vm_mgmt_get_verbose_class_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![0]) // false
}

pub(crate) fn vm_mgmt_get_verbose_gc_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![0]) // false
}

// ── Class-loading counters ────────────────────────────────────────────────────

pub(crate) fn vm_mgmt_get_loaded_class_count_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![0])
}

pub(crate) fn vm_mgmt_get_total_class_count_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(0))
}

pub(crate) fn vm_mgmt_get_unloaded_class_count_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(0))
}

// ── Thread counters ───────────────────────────────────────────────────────────

pub(crate) fn vm_mgmt_get_live_thread_count_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![1])
}

pub(crate) fn vm_mgmt_get_peak_thread_count_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![1])
}

pub(crate) fn vm_mgmt_get_daemon_thread_count_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![0])
}

pub(crate) fn vm_mgmt_get_total_thread_count_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(1))
}

// ── Safepoint / compile counters (stubs) ─────────────────────────────────────

pub(crate) fn vm_mgmt_get_total_compile_time_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(0))
}

pub(crate) fn vm_mgmt_get_safepoint_count_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(0))
}

pub(crate) fn vm_mgmt_get_total_safepoint_time_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(0))
}

pub(crate) fn vm_mgmt_get_safepoint_sync_time_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(0))
}

pub(crate) fn vm_mgmt_get_total_app_non_stopped_time_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(0))
}

// ── Class-metadata size counters (stubs) ─────────────────────────────────────

pub(crate) fn vm_mgmt_get_loaded_class_size_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(0))
}

pub(crate) fn vm_mgmt_get_unloaded_class_size_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(0))
}

pub(crate) fn vm_mgmt_get_class_loading_time_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(0))
}

pub(crate) fn vm_mgmt_get_method_data_size_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(0))
}

pub(crate) fn vm_mgmt_get_initialized_class_count_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(0))
}

pub(crate) fn vm_mgmt_get_class_initialization_time_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(0))
}

pub(crate) fn vm_mgmt_get_class_verification_time_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(0))
}

// ── Thread-monitoring enable/disable flags ────────────────────────────────────

pub(crate) fn vm_mgmt_is_thread_contention_monitoring_enabled_wrp(
    _args: &[i32],
) -> Result<Vec<i32>> {
    Ok(vec![0]) // false
}

pub(crate) fn vm_mgmt_is_thread_cpu_time_enabled_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![0]) // false
}

pub(crate) fn vm_mgmt_is_thread_allocated_memory_enabled_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![0]) // false
}

// ─── MemoryImpl ──────────────────────────────────────────────────────────────

const HEAP_MAX_BYTES: i64 = 256 * 1024 * 1024; // 256 MB stub

/// Returns a `MemoryUsage` object for the heap (`is_heap == 1`) or
/// non-heap (`is_heap == 0`) memory area.
pub(crate) fn memory_impl_get_memory_usage0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _self_ref = args[0];
    let is_heap = args[1] != 0;
    let (init, used, committed, max): (i64, i64, i64, i64) = if is_heap {
        (HEAP_MAX_BYTES, 0, HEAP_MAX_BYTES, HEAP_MAX_BYTES)
    } else {
        (0, 0, 0, -1) // non-heap: unknown/unlimited
    };
    let memory_usage_ref = Executor::invoke_args_constructor(
        "java/lang/management/MemoryUsage",
        "<init>:(JJJJ)V",
        &[init.into(), used.into(), committed.into(), max.into()],
        Some("MemoryImpl.getMemoryUsage0"),
    )?;
    Ok(vec![memory_usage_ref])
}

/// Returns an empty array of `MemoryPoolMXBean` (no GC pool management).
pub(crate) fn memory_impl_get_memory_pools0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    let array_ref = HEAP.create_array("[Ljava/lang/management/MemoryPoolMXBean;", 0);
    Ok(vec![array_ref])
}

/// Returns an empty array of `MemoryManagerMXBean` (no GC pool management).
pub(crate) fn memory_impl_get_memory_managers0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    let array_ref = HEAP.create_array("[Ljava/lang/management/MemoryManagerMXBean;", 0);
    Ok(vec![array_ref])
}

pub(crate) fn memory_impl_set_verbose_gc_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![])
}

// ─── MemoryManagerImpl ───────────────────────────────────────────────────────

pub(crate) fn memory_manager_impl_get_memory_pools0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    let array_ref = HEAP.create_array("[Ljava/lang/management/MemoryPoolMXBean;", 0);
    Ok(vec![array_ref])
}

// ─── ClassLoadingImpl ────────────────────────────────────────────────────────

pub(crate) fn class_loading_impl_set_verbose_class_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![])
}

// ─── ThreadImpl ──────────────────────────────────────────────────────────────

/// Returns an array containing only the current (primordial) thread.
pub(crate) fn thread_impl_get_threads_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let thread_ref = current_thread_wrp(args)?[0];
    let array_ref = HEAP.create_array_with_values("[Ljava/lang/Thread;", &[thread_ref]);
    Ok(vec![array_ref])
}

/// Populates `ThreadInfo` entries.  We have only one thread; fill in
/// its info using the same thread ref we return in `getThreads()`.
pub(crate) fn thread_impl_get_thread_info1_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    // Signature: ([JI[Ljava/lang/management/ThreadInfo;)V
    // We leave the ThreadInfo entries as null (already zero-initialised),
    // which is acceptable for a single-threaded VM.
    Ok(vec![])
}

// Remaining ThreadImpl stubs
pub(crate) fn thread_impl_reset_contention_times0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![])
}

pub(crate) fn thread_impl_set_thread_contention_monitoring_enabled0_wrp(
    _args: &[i32],
) -> Result<Vec<i32>> {
    Ok(vec![])
}

pub(crate) fn thread_impl_get_thread_total_cpu_time0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(-1))
}

pub(crate) fn thread_impl_get_thread_total_cpu_time1_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![])
}

pub(crate) fn thread_impl_get_thread_user_cpu_time0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(-1))
}

pub(crate) fn thread_impl_get_thread_user_cpu_time1_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![])
}

pub(crate) fn thread_impl_set_thread_cpu_time_enabled0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![])
}

pub(crate) fn thread_impl_get_total_thread_allocated_memory_wrp(
    _args: &[i32],
) -> Result<Vec<i32>> {
    Ok(i64_to_vec(-1))
}

pub(crate) fn thread_impl_get_thread_allocated_memory0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(-1))
}

pub(crate) fn thread_impl_get_thread_allocated_memory1_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![])
}

pub(crate) fn thread_impl_set_thread_allocated_memory_enabled0_wrp(
    _args: &[i32],
) -> Result<Vec<i32>> {
    Ok(vec![])
}

pub(crate) fn thread_impl_find_monitor_deadlocked_threads0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![0]) // null – no deadlocks
}

pub(crate) fn thread_impl_find_deadlocked_threads0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![0]) // null – no deadlocks
}

pub(crate) fn thread_impl_reset_peak_thread_count0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![])
}

pub(crate) fn thread_impl_dump_threads0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    // Returns ThreadInfo[] – return empty array
    let array_ref = HEAP.create_array("[Ljava/lang/management/ThreadInfo;", 0);
    Ok(vec![array_ref])
}

// ─── GarbageCollectorImpl ────────────────────────────────────────────────────

pub(crate) fn gc_impl_get_collection_count_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(0))
}

pub(crate) fn gc_impl_get_collection_time_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(0))
}

// ─── com.sun.management.internal.OperatingSystemImpl ─────────────────────────

pub(crate) fn os_impl_initialize0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![])
}

pub(crate) fn os_impl_get_committed_virtual_memory_size0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(0))
}

pub(crate) fn os_impl_get_total_swap_space_size0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(0))
}

pub(crate) fn os_impl_get_free_swap_space_size0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(0))
}

pub(crate) fn os_impl_get_process_cpu_time0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(-1))
}

/// Returns a stub free physical memory size (128 MB).
pub(crate) fn os_impl_get_free_memory_size0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(128 * 1024 * 1024))
}

/// Returns a stub total physical memory size (256 MB).
pub(crate) fn os_impl_get_total_memory_size0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(256 * 1024 * 1024))
}

pub(crate) fn os_impl_get_open_fd_count0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(0))
}

pub(crate) fn os_impl_get_max_fd_count0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(1024))
}

pub(crate) fn os_impl_get_cpu_load0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(zero_double())
}

pub(crate) fn os_impl_get_process_cpu_load0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(zero_double())
}

pub(crate) fn os_impl_get_host_online_cpu_count0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![STUB_CPU_COUNT])
}

pub(crate) fn os_impl_get_single_cpu_load0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(zero_double())
}

pub(crate) fn os_impl_get_host_configured_cpu_count0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(vec![STUB_CPU_COUNT])
}

pub(crate) fn os_impl_get_host_total_cpu_ticks0_wrp(_args: &[i32]) -> Result<Vec<i32>> {
    Ok(i64_to_vec(0))
}
