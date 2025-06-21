//! # jimage-rs
//! A fast and efficient implementation from scratch of Rust library for dealing with Java Modular System (`jimage`) files.
//! ## Example
//! ```rust
//! use std::env;
//! use jimage_rs::JImage;
//!
//! fn main() {
//!     let path = format!("{}/lib/modules", env::var("JAVA_HOME").unwrap());
//!     let jimage = JImage::open(path).unwrap();
//!     let resource = jimage.find_resource("/java.base/java/lang/String.class");
//!     println!("Resource: {:?}", resource);
//! }
//! ```
mod bytes_utils;
pub mod error;
mod header;
pub mod jimage;
pub use jimage::JImage;
