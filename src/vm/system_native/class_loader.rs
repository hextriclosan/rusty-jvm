use crate::vm::error::Result;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::{clazz_ref, vec_to_i64};
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
    let buf = HEAP.get_entire_array(buf_ref)?;

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

pub(crate) fn define_class2_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let class_loader_ref = args[0];
    let name_ref = args[1];
    let byte_buf_ref = args[2];
    let off = args[3];
    let len = args[4];
    let protection_domain_ref = args[5];
    let source_ref = args[6];
    let class_ref = define_class2(
        class_loader_ref,
        name_ref,
        byte_buf_ref,
        off,
        len,
        protection_domain_ref,
        source_ref,
    )?;

    Ok(vec![class_ref])
}

fn define_class2(
    _class_loader_ref: i32,
    name_ref: i32,
    byte_buf_ref: i32,
    off: i32,
    len: i32,
    _protection_domain_ref: i32,
    _source_ref: i32,
) -> Result<i32> {
    let internal_name = get_utf8_string_by_ref(name_ref)?.replace(".", "/");
    let instance_name = HEAP.get_instance_name(byte_buf_ref)?;
    let addr = HEAP.get_object_field_value(byte_buf_ref, &instance_name, "address")?;
    let addr = vec_to_i64(&addr);

    let addr = addr as usize as *const u8;
    if addr.is_null() {
        return Err(crate::vm::error::Error::new_execution(
            "ByteBuffer address is null",
        ));
    }
    let byte_code = unsafe { std::slice::from_raw_parts(addr.add(off as usize), len as usize) };

    let (name, ..) =
        with_method_area(|method_area| method_area.create_metaclass(&internal_name, &byte_code))?;
    let clazz_ref = clazz_ref(&name);

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

    // Check if the class is already loaded and exists
    // If it does not exist, we return 0 (null reference)
    if let Err(_) = with_method_area(|a| a.get(internal_name)) {
        return Ok(0);
    }

    clazz_ref(internal_name)
}

pub(crate) fn find_loaded_class_wrp(args: &[i32]) -> Result<Vec<i32>> {
    let _class_loader_ref = args[0];
    let name_ref = args[1];
    let clazz_ref = find_loaded_class(name_ref)?;

    Ok(vec![clazz_ref])
}
fn find_loaded_class(name_ref: i32) -> Result<i32> {
    let name = get_utf8_string_by_ref(name_ref)?;
    let internal_name = &name.replace('.', "/");
    if let Some(_) = with_method_area(|a| a.get_only_loaded(internal_name))? {
        clazz_ref(internal_name)
    } else {
        Ok(0)
    }
}

fn increment_counter() -> u32 {
    COUNTER.fetch_add(1, Ordering::SeqCst)
}
