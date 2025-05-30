/*!
# `jdescriptor` - A Java Bytecode Descriptor Parser for Rust

`jdescriptor` is a Rust library for parsing and formatting Java bytecode descriptors.
It provides robust support for handling Java type and method descriptors while leveraging Rust’s strong type system and error-handling capabilities.

## Features

- **Comprehensive Descriptor Parsing**
  - Supports Java type descriptors, including primitive types, object types, and arrays.
  - Parses Java method descriptors, extracting parameter types and return types.

- **String Representations**
  - Implements `Display` for converting parsed descriptors back into Java-style string representations.

- **Error Handling**
  - Provides structured error types (`DescriptorError`) for clear and precise error reporting.

- **Idiomatic Rust API**
  - Implements `FromStr` for easy descriptor parsing using standard Rust conventions.

- **Well-Tested**
  - Includes extensive unit tests to ensure correctness and reliability.
*/

use crate::TypeDescriptor::*;
use std::fmt;
use std::fmt::Display;
use std::str::{Chars, FromStr};
use thiserror::Error;

/// Custom error types for descriptor parsing.
#[derive(Debug, PartialEq, Error)]
pub enum DescriptorError {
    #[error("Invalid descriptor: {0}")]
    InvalidFormat(&'static str),
    #[error("Unexpected end of input.")]
    UnexpectedEndOfInput,
    #[error("Array has too many dimensions.")]
    TooManyDimensions,
}

/** Represents a Java type descriptor, which can be a primitive type, an array, or an object type.

## Example Usage

### Parsing a Type Descriptor

```rust
use jdescriptor::TypeDescriptor;
use std::str::FromStr;

let descriptor = TypeDescriptor::from_str("Ljava/lang/String;").unwrap();
assert_eq!(descriptor.to_string(), "Ljava/lang/String;");
```
*/
#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize)]
pub enum TypeDescriptor {
    Byte,
    Char,
    Double,
    Float,
    Integer,
    Long,
    Short,
    Boolean,
    Void,
    Array(Box<TypeDescriptor>, u8),
    Object(String),
}

/** Represents a Java method descriptor consisting of parameter types and a return type.

## Example Usage

### Parsing a Method Descriptor

```rust
use jdescriptor::MethodDescriptor;
use std::str::FromStr;

let descriptor = MethodDescriptor::from_str("([[[Ljava/lang/String;)[[[I").unwrap();
assert_eq!(descriptor.to_string(), "([[[Ljava/lang/String;)[[[I");
```
*/
#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize)]
pub struct MethodDescriptor {
    parameter_types: Vec<TypeDescriptor>,
    return_type: TypeDescriptor,
}

/// Implements the `Display` trait for `TypeDescriptor` to convert it back to a string representation.
impl Display for TypeDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Byte => write!(f, "B"),
            Char => write!(f, "C"),
            Double => write!(f, "D"),
            Float => write!(f, "F"),
            Integer => write!(f, "I"),
            Long => write!(f, "J"),
            Short => write!(f, "S"),
            Boolean => write!(f, "Z"),
            Void => write!(f, "V"),
            Array(base_type, dimensions) => {
                write!(f, "{}", "[".repeat(*dimensions as usize))?;
                write!(f, "{}", base_type)
            }
            Object(class_name) => write!(f, "L{};", class_name),
        }
    }
}

/// Implements the `Display` trait for `MethodDescriptor` to convert it back to a string representation.
impl Display for MethodDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        for param in &self.parameter_types {
            write!(f, "{}", param)?;
        }
        write!(f, "){}", self.return_type)
    }
}

/// Implements the `Display` trait for `MethodDescriptor` to convert it back to a string representation.
impl MethodDescriptor {
    pub fn new(parameter_types: Vec<TypeDescriptor>, return_type: TypeDescriptor) -> Self {
        Self {
            parameter_types,
            return_type,
        }
    }

    /// Returns a reference to the parameter types.
    pub fn parameter_types(&self) -> &Vec<TypeDescriptor> {
        &self.parameter_types
    }

    /// Returns a reference to the return type.
    pub fn return_type(&self) -> &TypeDescriptor {
        &self.return_type
    }
}

/// Implements the `Display` trait for `MethodDescriptor` to convert it back to a string representation.
impl FromStr for TypeDescriptor {
    type Err = DescriptorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        get_next(&mut s.chars())?.ok_or(DescriptorError::InvalidFormat("Invalid descriptor"))
    }
}

/// Parses a type descriptor from a string representation.
fn get_next(chars: &mut Chars) -> Result<Option<TypeDescriptor>, DescriptorError> {
    let result = match chars.next().ok_or(DescriptorError::UnexpectedEndOfInput)? {
        'Z' => Some(Boolean),
        'B' => Some(Byte),
        'C' => Some(Char),
        'S' => Some(Short),
        'I' => Some(Integer),
        'J' => Some(Long),
        'F' => Some(Float),
        'D' => Some(Double),
        'V' => Some(Void),
        'L' => {
            let mut class_name = String::new();
            for c in chars {
                if c == ';' {
                    return Ok(Some(Object(class_name)));
                }
                class_name.push(c);
            }
            return Err(DescriptorError::InvalidFormat(
                "Missing semicolon in class name descriptor.",
            ));
        }
        '[' => {
            let mut dimensions = 1u8;
            while let Some('[') = chars.clone().next() {
                chars.next();
                dimensions = dimensions
                    .checked_add(1)
                    .ok_or(DescriptorError::TooManyDimensions)?;
            }
            let base_type =
                get_next(chars)?.ok_or(DescriptorError::InvalidFormat("Invalid descriptor."))?;
            Some(Array(Box::new(base_type), dimensions))
        }
        ')' => None,
        _ => {
            return Err(DescriptorError::InvalidFormat(
                "Unrecognized type descriptor.",
            ))
        }
    };

    Ok(result)
}

/// Parses a method descriptor from a string representation.
impl FromStr for MethodDescriptor {
    type Err = DescriptorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        if chars.next() != Some('(') {
            return Err(DescriptorError::InvalidFormat(
                "Method descriptor must start with '('.",
            ));
        }

        let mut parameter_types = Vec::new();
        while let Some(descr) = get_next(&mut chars)? {
            parameter_types.push(descr);
        }

        let return_type =
            get_next(&mut chars)?.ok_or(DescriptorError::InvalidFormat("Missing return type."))?;
        Ok(Self::new(parameter_types, return_type))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Z", Boolean)]
    #[case("B", Byte)]
    #[case("C", Char)]
    #[case("S", Short)]
    #[case("I", Integer)]
    #[case("J", Long)]
    #[case("F", Float)]
    #[case("D", Double)]
    #[case("V", Void)]
    #[case("[I", Array(Box::new(Integer), 1))]
    #[case("[[I", Array(Box::new(Integer), 2))]
    #[case("[[[Ljava/lang/String;", Array(Box::new(Object("java/lang/String".to_string())), 3))]
    #[case(&("[".repeat(255) + "I"), Array(Box::new(Integer), 255))]
    #[case("Ljava/lang/Object;", Object("java/lang/Object".to_string()))]
    fn should_parse_and_then_convert_to_string_type_descriptor(
        #[case] input: &str,
        #[case] expected: TypeDescriptor,
    ) {
        let parsed = str::parse::<TypeDescriptor>(input).unwrap();
        assert_eq!(parsed, expected);
        assert_eq!(parsed.to_string(), input);
    }

    #[rstest]
    #[case("(II)I", MethodDescriptor::new(vec![Integer, Integer], Integer))] // int add(int a, int b)
    #[case("([Ljava/lang/String;)V", MethodDescriptor::new(vec![Array(Box::new(Object("java/lang/String".to_string())), 1)], Void))] // void main(String[] args)
    #[case("([I[I)[I", MethodDescriptor::new(vec![Array(Box::new(Integer), 1), Array(Box::new(Integer), 1)], Array(Box::new(Integer), 1)))] // int[] combineArrays(int[], int[])
    #[case("([[[Ljava/lang/String;)[[[I", MethodDescriptor::new(vec![Array(Box::new(Object("java/lang/String".to_string())), 3)], Array(Box::new(Integer), 3)))] // int[][][] process(String[][][] data)
    #[case("([Ljava/util/concurrent/ConcurrentHashMap$Node;ILjava/util/concurrent/ConcurrentHashMap$Node;Ljava/util/concurrent/ConcurrentHashMap$Node;)Z", MethodDescriptor::new(vec![Array(Box::new(Object("java/util/concurrent/ConcurrentHashMap$Node".to_string())), 1), Integer, Object("java/util/concurrent/ConcurrentHashMap$Node".to_string()), Object("java/util/concurrent/ConcurrentHashMap$Node".to_string())], Boolean))] // boolean casTabAt(Node<K,V>[] tab, int i, Node<K,V> c, Node<K,V> v)
    #[case("(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;", MethodDescriptor::new(vec![Object("java/lang/Object".to_string()), Object("java/lang/Object".to_string()), Object("java/util/function/BiFunction".to_string())], Object("java/lang/Object".to_string())))] // V merge(K, V, java.util.function.BiFunction<? super V, ? super V, ? extends V>)
    fn should_parse_and_then_convert_to_string_method_descriptor(
        #[case] input: &str,
        #[case] expected: MethodDescriptor,
    ) {
        let parsed = str::parse::<MethodDescriptor>(input).unwrap();
        assert_eq!(parsed, expected);
        assert_eq!(parsed.to_string(), input);
    }

    #[test]
    fn should_return_error_for_unsupported_type() {
        assert_eq!(
            str::parse::<TypeDescriptor>("X"),
            Err(DescriptorError::InvalidFormat(
                "Unrecognized type descriptor."
            ))
        );
    }
    #[test]
    fn should_return_error_for_array_without_type() {
        assert_eq!(
            str::parse::<TypeDescriptor>("["),
            Err(DescriptorError::UnexpectedEndOfInput)
        );
    }
    #[test]
    fn should_return_error_for_object_without_closing_symbol() {
        assert_eq!(
            str::parse::<TypeDescriptor>("Ljava/lang/String"),
            Err(DescriptorError::InvalidFormat(
                "Missing semicolon in class name descriptor."
            ))
        );
    }
    #[test]
    fn should_return_error_for_array_with_too_many_dimensions() {
        let mut type_descriptor = String::new();
        type_descriptor.push_str("[".repeat(256usize).as_str());
        type_descriptor.push_str("I");
        assert_eq!(
            str::parse::<TypeDescriptor>(type_descriptor.as_str()),
            Err(DescriptorError::TooManyDimensions)
        );
    }
}
