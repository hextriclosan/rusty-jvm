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
//!
//!     let resource_count = jimage.resource_names_iter().count();
//!     println!("Total resources in jimage: {}", resource_count);
//!
//!     match jimage.find_resource("/java.base/java/lang/String.class")? {
//!         Some(resource) => println!("Resource found, its size is {} bytes", resource.len()),
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
pub mod raw_jimage;
mod resource_header;
mod resource_name;
pub use crate::jimage::JImage;
pub use crate::resource_name::{ResourceName, ResourceNamesIter};
