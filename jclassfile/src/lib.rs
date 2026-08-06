//! A faithful parser for Java class files, following the
//! [Java Virtual Machine Specification (JVMS), Chapter §4][jvms-4].
//!
//! Start at [`class_file::parse`], which turns bytes into a [`class_file::ClassFile`] mirroring
//! JVMS §4 one-for-one.
//!
//! ```no_run
//! let bytecode = std::fs::read("HelloWorld.class")?;
//! let class_file = jclassfile::class_file::parse(&bytecode)?;
//!
//! println!("{class_file:?}");
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Looking for resolved values? See [`jclassmodel`]
//!
//! This crate is deliberately faithful rather than convenient. Constant pool entries hold raw
//! `u16` indexes into each other, attributes come back as an untyped `Vec<Attribute>`, and a name
//! is reachable only by following an index to a `Utf8` entry. That is the right shape for a
//! parser: it loses nothing and decides nothing on your behalf.
//!
//! It is not the shape most consumers want. Reading a method's name means resolving
//! `name_index`; finding its `Code` means scanning a list of attributes; interpreting an exception
//! handler means resolving `catch_type` and knowing that zero means catch-all. Every project that
//! consumes class files ends up writing that same layer.
//!
//! [`jclassmodel`] is that layer, built on this crate. It resolves every index, groups attributes
//! by kind, parses descriptors, and hands back a view meant for use rather than inspection.
//!
//! - Reach for **`jclassfile`** when you want the file exactly as the spec describes it: a
//!   disassembler, a verifier, a class file rewriter, anything that must not lose a byte.
//! - Reach for **`jclassmodel`** when you want to *use* the class: a JVM, a static analyser, a
//!   reflection layer.
//!
//! `jclassmodel` keeps this crate as a private dependency, so it does not appear in that crate's
//! public API and the two version independently. Nothing here depends on `jclassmodel`; the
//! reference is one-way.
//!
//! [jvms-4]: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html
//! [`jclassmodel`]: https://crates.io/crates/jclassmodel

pub mod attributes;
pub mod class_file;
pub mod constant_pool;
pub mod error;
mod extractors;
pub mod fields;
pub mod methods;
