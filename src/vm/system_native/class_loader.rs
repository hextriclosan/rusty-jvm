use crate::vm::error::Result;
use crate::vm::heap::heap::with_heap_read_lock;
use crate::vm::helper::clazz_ref;
use crate::vm::method_area::method_area::with_method_area;
use crate::vm::system_native::string::get_utf8_string_by_ref;
use std::sync::atomic::{AtomicU32, Ordering};

pub(crate) const SYNTH_CLASS_DELIM: char = '#';
static COUNTER: AtomicU32 = AtomicU32::new(1);

pub(crate) fn define_class0_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let class_loader_ref = args[0];
    let lookup_ref = args[1];
    let name_ref = args[2];
    let buf_ref = args[3];
    let off = args[4];
    let len = args[5];
    let protection_domain_ref = args[6];
    let initialize = args[7] != 0;
    let flags = args[8];
    let class_data_ref = args[9];
    let class_ref = define_class0(
        class_loader_ref,
        lookup_ref,
        name_ref,
        buf_ref,
        off,
        len,
        protection_domain_ref,
        initialize,
        flags,
        class_data_ref,
    )?;

    Ok(vec![class_ref])
}
fn define_class0(
    _class_loader_ref: i32,
    _lookup_ref: i32,
    name_ref: i32,
    buf_ref: i32,
    off: i32,
    len: i32,
    _protection_domain_ref: i32,
    _initialize: bool,
    _flags: i32,
    _class_data_ref: i32,
) -> Result<i32> {
    let name = format!(
        "{}{SYNTH_CLASS_DELIM}0x{:016X}",
        get_utf8_string_by_ref(name_ref)?,
        increment_counter()
    );
    let buf = with_heap_read_lock(|heap| heap.get_entire_array(buf_ref))?;

    let vec = buf.get_entire_value();
    let byte_code: Vec<_> = vec
        .iter()
        .skip(off as usize)
        .take(len as usize)
        .map(|v| v[0] as u8)
        .collect();

    let (internal_name, ..) =
        with_method_area(|method_area| method_area.create_metaclass(&name, &byte_code))?;
    let clazz_ref = clazz_ref(&internal_name);

    clazz_ref
}

pub(crate) fn find_bootstrap_class_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let name_ref = args[0];
    let clazz_ref = find_bootstrap_class(name_ref)?;

    Ok(vec![clazz_ref])
}
fn find_bootstrap_class(name_ref: i32) -> Result<i32> {
    let name = get_utf8_string_by_ref(name_ref)?;
    let internal_name = &name.replace('.', "/");
    clazz_ref(internal_name)
}

fn increment_counter() -> u32 {
    COUNTER.fetch_add(1, Ordering::SeqCst)
}
