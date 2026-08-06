//! A runtime-friendly view over Java class files parsed by [`jclassfile`].
//!
//! `jclassfile` produces a structure that mirrors JVMS §4 exactly, with cross-references left as
//! raw constant-pool indexes. This crate resolves those indexes and reorganizes the result for
//! lookup, so consumers do not each reimplement the same postprocessing.
//!
//! # Example
//!
//! Start at [`parse`], then [`ParsedClass::into_model_with_recorded_name`] to get a
//! [`ClassModel`]. From there everything is resolved: no constant-pool index appears again.
//!
//! ```no_run
//! use jclassmodel::{parse, MethodModifier};
//!
//! let bytecode = std::fs::read("Example.class")?;
//! let model = parse(&bytecode)?.into_model_with_recorded_name()?;
//!
//! println!("{} (class file version {})", model.external_name, model.version);
//! if let Some(super_class_name) = &model.super_class_name {
//!     println!("  extends {super_class_name}");
//! }
//! for interface in &model.interfaces {
//!     println!("  implements {interface}");
//! }
//!
//! for field in &model.fields {
//!     println!("  field {}: {}", field.name, field.descriptor);
//! }
//!
//! for method in &model.methods {
//!     println!("  method {} returning {}", method.name, method.descriptor.return_type());
//!
//!     if method.modifiers.contains(MethodModifier::Static) {
//!         println!("    static");
//!     }
//!     for exception in &method.exceptions {
//!         println!("    throws {exception}");
//!     }
//!     if let Some(code) = &method.code {
//!         println!("    {} bytes of bytecode", code.bytecode.len());
//!
//!         // Line numbers are keyed by bytecode index, so a stack trace is a lookup away.
//!         if let Some((_, line)) = code.line_numbers.iter().next() {
//!             println!("    starts at source line {line}");
//!         }
//!     }
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! Pass the names yourself with [`ParsedClass::into_model`] when the class is loaded under a name
//! its own constant pool does not record, which is what a JVM does for hidden classes.
//!
//! # Dependencies that are, and are not, part of the API
//!
//! `jclassfile` is a private dependency: it does not appear anywhere in this crate's public API,
//! so depending on this crate does not tie you to a particular `jclassfile` version.
//!
//! [`jdescriptor`] is the opposite, deliberately: [`MethodModel::descriptor`] and
//! [`FieldModel::descriptor`] hand back its [`MethodDescriptor`] and [`TypeDescriptor`], because
//! parsed descriptors are vocabulary a consumer needs to pass around rather than an implementation
//! detail worth hiding. Its version is therefore part of this crate's semver contract: a
//! `jdescriptor` release that breaks compatibility breaks this crate's API too.
//!
//! One consequence is worth spelling out. Rust type identity is per compiled copy of a crate, so
//! if your build resolves a second, semver-incompatible `jdescriptor`, its `MethodDescriptor` is a
//! *different type* from the one here, and values cannot cross the boundary. To make that
//! impossible, `jdescriptor` is re-exported: reach for [`jclassmodel::MethodDescriptor`] or the
//! whole crate at [`jclassmodel::jdescriptor`] rather than declaring your own dependency on it.
//!
//! ```
//! use jclassmodel::{MethodDescriptor, TypeDescriptor};
//! // or, for anything else the crate offers:
//! use jclassmodel::jdescriptor;
//! ```
//!
//! [`jclassfile`]: https://crates.io/crates/jclassfile
//! [`jdescriptor`]: https://crates.io/crates/jdescriptor
//! [`jclassmodel::MethodDescriptor`]: MethodDescriptor
//! [`jclassmodel::jdescriptor`]: jdescriptor

pub mod attributes;
pub mod class_model;
pub mod constant_pool;
pub mod error;
pub mod exception_table;
pub mod modifiers;

pub use attributes::{Attributes, BootstrapMethodInfo, LocalVariableInfo};
pub use class_model::{
    parse, ClassFileVersion, ClassModel, CodeModel, EnclosingMethodInfo, EnclosingMethodRef,
    FieldModel, MethodModel, ParameterInfo, ParsedClass,
};
pub use constant_pool::{ConstantPool, ConstantPoolLookup, ConstantTag};
pub use error::{Error, ErrorKind, Result};
pub use exception_table::{CatchType, ExceptionTableRecord};
/// Re-exported because it is part of this crate's public API: descriptors handed back by
/// [`MethodModel`] and [`FieldModel`] are these types, and a second copy of `jdescriptor` in the
/// build graph would make them incompatible. Use these rather than depending on `jdescriptor`
/// directly.
pub use jdescriptor::{self, MethodDescriptor, TypeDescriptor};
pub use modifiers::{
    ClassModifier, FieldModifier, MethodModifier, NestedClassModifier, ParameterModifier,
};
