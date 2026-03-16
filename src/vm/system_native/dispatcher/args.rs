use jdescriptor::TypeDescriptor;
use libffi::middle::Arg;
use std::ffi::c_void;

pub(super) enum ArgStorage {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    I8(i8),
    I16(i16),
    U16(u16),
    U8(u8),
    Ptr(*mut c_void),
}

impl ArgStorage {
    pub(super) fn as_arg(&self) -> Arg<'_> {
        match self {
            ArgStorage::I32(v) => Arg::new(v),
            ArgStorage::I64(v) => Arg::new(v),
            ArgStorage::F32(v) => Arg::new(v),
            ArgStorage::F64(v) => Arg::new(v),
            ArgStorage::I8(v) => Arg::new(v),
            ArgStorage::I16(v) => Arg::new(v),
            ArgStorage::U16(v) => Arg::new(v),
            ArgStorage::U8(v) => Arg::new(v),
            ArgStorage::Ptr(v) => Arg::new(v),
        }
    }
}

pub(super) fn build_args(
    args: &[i32],
    types: &[TypeDescriptor],
    is_static: bool,
) -> Vec<ArgStorage> {
    let mut storage = Vec::with_capacity(types.len());

    let mut i = 0;

    if !is_static {
        let obj_ref = args[i];
        i += 1;
        let ptr = obj_ref as usize as *mut c_void;
        storage.push(ArgStorage::Ptr(ptr));
    }

    // todo: rework by analogy to StackValue/StackValueKind
    for t in types {
        match t {
            TypeDescriptor::Integer => {
                let v = args[i];
                storage.push(ArgStorage::I32(v));
                i += 1;
            }

            TypeDescriptor::Float => {
                let v = f32::from_bits(args[i] as u32);
                storage.push(ArgStorage::F32(v));
                i += 1;
            }

            TypeDescriptor::Byte => {
                storage.push(ArgStorage::I8(args[i] as i8));
                i += 1;
            }

            TypeDescriptor::Short => {
                storage.push(ArgStorage::I16(args[i] as i16));
                i += 1;
            }

            TypeDescriptor::Char => {
                storage.push(ArgStorage::U16(args[i] as u16));
                i += 1;
            }

            TypeDescriptor::Boolean => {
                storage.push(ArgStorage::U8(args[i] as u8));
                i += 1;
            }

            TypeDescriptor::Long => {
                let low = args[i] as u32;
                let high = args[i + 1] as u32;
                let v = ((high as u64) << 32 | (low as u64)) as i64;

                storage.push(ArgStorage::I64(v));
                i += 2;
            }

            TypeDescriptor::Double => {
                let low = args[i] as u32;
                let high = args[i + 1] as u32;
                let bits = (high as u64) << 32 | low as u64;

                storage.push(ArgStorage::F64(f64::from_bits(bits)));
                i += 2;
            }

            TypeDescriptor::Object(_) | TypeDescriptor::Array(_, _) => {
                let ptr = args[i] as usize as *mut c_void;
                storage.push(ArgStorage::Ptr(ptr));
                i += 1;
            }

            TypeDescriptor::Void => unreachable!(),
        }
    }

    storage
}
