//! # jimage-rs
//! A fast and efficient Rust library for working with `jimage` files used by the Java Platform Module System.
//! ## Example
//! ```rust
//! use std::env;
//! use std::path::PathBuf;
//! use jimage_rs::JImage;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let path = PathBuf::from(env::var("JAVA_HOME")?)
//!         .join("lib")
//!         .join("modules");
//!     let jimage = JImage::open(path)?;
//!     match jimage.find_resource("/java.base/java/lang/String.class")? {
//!         Some(resource) => println!("Resource found: {:?}", resource),
//!         None => println!("Resource not found"),
//!     }
//!
//!     Ok(())
//! }
//! ```
mod bytes_utils;
pub mod error;
mod header;
pub mod jimage;
mod resource_header;
pub use jimage::JImage;
