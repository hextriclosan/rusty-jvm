use crate::vm::error::Result;
use crate::vm::system_native::dispatcher::utils::to_ffitype;
use jdescriptor::MethodDescriptor;
use libffi::middle::{Cif, Type};
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::{Arc, LazyLock, Mutex};

pub(super) static CIF_CACHE: LazyLock<Mutex<LruCache<String, Arc<SafeCif>>>> =
    LazyLock::new(|| {
        Mutex::new(LruCache::new(
            NonZeroUsize::new(10000).expect("NonZeroUsize::new failed"),
        ))
    });
#[repr(transparent)]
pub(super) struct SafeCif(pub Cif);

// SAFETY: Cif is immutable after preparation and safe to share between threads
unsafe impl Send for SafeCif {}
unsafe impl Sync for SafeCif {}

pub(super) fn get_cif(method_descriptor: &MethodDescriptor) -> Result<Arc<SafeCif>> {
    let mut cache = CIF_CACHE.lock()?;
    let cif = cache.get_or_insert(method_descriptor.to_string(), || {
        let mut arg_types = vec![
            Type::pointer(), // JNIEnv*
            Type::pointer(), // jobject or jclass
        ];

        let from_descriptor = method_descriptor
            .parameter_types()
            .iter()
            .map(to_ffitype)
            .collect::<Vec<_>>();

        arg_types.extend(from_descriptor);

        let ret_type = to_ffitype(method_descriptor.return_type());
        let cif = Cif::new(arg_types, ret_type);
        Arc::new(SafeCif(cif))
    });
    Ok(Arc::clone(cif))
}
