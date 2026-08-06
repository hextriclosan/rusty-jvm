use derive_new::new;
use getset::{CopyGetters, Getters};

/// What an exception handler catches, resolved from the `catch_type` index of an
/// `exception_table` entry (JVMS §4.7.3).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CatchType {
    /// Catches every throwable. JVMS §4.7.3 records this as a `catch_type` of zero, which is what
    /// a `finally` block and `try`-with-resources compile to.
    Any,
    /// Catches this class and its subclasses, by internal name (`java/io/IOException`).
    Class(String),
}

/// An `exception_table` entry of a `Code` attribute (JVMS §4.7.3), with `catch_type` already
/// resolved from a constant-pool index.
///
/// This carries the data only. Deciding whether a record actually handles a given throwable needs
/// the loaded-class graph (to test assignability), so that logic belongs to the consumer:
///
/// ```
/// use jclassmodel::CatchType;
/// # fn is_assignable(_: &str, _: &str) -> bool { true }
/// # let (record_catch_type, thrown) = (CatchType::Any, "java/io/IOException");
/// let handles = match &record_catch_type {
///     CatchType::Any => true,
///     CatchType::Class(name) => is_assignable(thrown, name),
/// };
/// # assert!(handles);
/// ```
#[derive(Debug, Clone, new, PartialEq, Eq, Hash, CopyGetters, Getters)]
pub struct ExceptionTableRecord {
    #[get_copy = "pub"]
    start_pc: u16,
    #[get_copy = "pub"]
    end_pc: u16,
    #[get_copy = "pub"]
    handler_pc: u16,
    #[get = "pub"]
    catch_type: CatchType,
}
