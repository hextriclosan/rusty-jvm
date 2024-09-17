use crate::TypeDescriptor::*;
use std::fmt::Display;
use std::str::Chars;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum DescriptorError {
    UnexpectedEndOfInput,
    InvalidDescriptor,
    UnrecognizedTypeDescriptor,
    MissingSemicolonInClassNameDescriptor,
    TooManyDimensions,
}

#[derive(Debug, PartialEq)]
pub enum TypeDescriptor {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Boolean,
    Void,
    Array(Box<TypeDescriptor>, u8),
    Object(String),
}

#[derive(Debug, PartialEq)]
pub struct MethodDescriptor {
    parameter_types: Vec<TypeDescriptor>,
    arguments_length: usize,
    return_type: TypeDescriptor,
}

pub fn default_value(type_descriptor: &TypeDescriptor) -> Vec<i32> {
    match type_descriptor {
        Byte | Char | Int | Short | Boolean => vec![0],
        Long => vec![0, 0],
        Float => todo!(),
        Double => todo!(),
        Void => panic!("field can't be a void type"),
        Array(_, _) => vec![0],
        Object(_) => todo!(),
    }
}

pub fn get_length(type_descriptor: &TypeDescriptor) -> usize {
    match type_descriptor {
        Byte | Char | Int | Short | Boolean | Float => 1,
        Long | Double => 2,
        Void => panic!("field can't be a void type"),
        Array(_, _) => 1,
        Object(_) => todo!(),
    }
}

impl MethodDescriptor {
    pub fn new(parameter_types: Vec<TypeDescriptor>, return_type: TypeDescriptor) -> Self {
        let arguments_length = calculate_arguments_length(&parameter_types);
        Self {
            parameter_types,
            arguments_length,
            return_type,
        }
    }

    pub fn parameter_types(&self) -> &Vec<TypeDescriptor> {
        &self.parameter_types
    }

    pub fn arguments_length(&self) -> usize {
        self.arguments_length
    }

    pub fn return_type(&self) -> &TypeDescriptor {
        &self.return_type
    }
}

fn calculate_arguments_length(parameter_types: &Vec<TypeDescriptor>) -> usize {
    parameter_types
        .iter()
        .map(|t| match t {
            Long | Double => 2,
            _ => 1,
        })
        .sum()
}

impl FromStr for TypeDescriptor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        get_next(&mut s.chars())
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Invalid descriptor".to_string())
    }
}

fn get_next(chars: &mut Chars) -> Result<Option<TypeDescriptor>, DescriptorError> {
    let result = match chars.next().ok_or(DescriptorError::UnexpectedEndOfInput)? {
        'Z' => Some(Boolean),
        'B' => Some(Byte),
        'C' => Some(Char),
        'S' => Some(Short),
        'I' => Some(Int),
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
            return Err(DescriptorError::MissingSemicolonInClassNameDescriptor);
        }
        '[' => {
            let mut dimensions = 1u8;
            while let Some('[') = chars.clone().next() {
                chars.next();
                dimensions = dimensions
                    .checked_add(1)
                    .ok_or(DescriptorError::TooManyDimensions)?;
            }
            let base_type = get_next(chars)?.ok_or(DescriptorError::InvalidDescriptor)?;
            Some(Array(Box::new(base_type), dimensions))
        }
        ')' => None,
        _ => return Err(DescriptorError::UnrecognizedTypeDescriptor),
    };

    Ok(result)
}

impl FromStr for MethodDescriptor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        if chars.next() != Some('(') {
            return Err("Method descriptor must start with '('".to_string());
        }

        let mut parameter_types = Vec::new();
        while let Some(descr) = get_next(&mut chars).map_err(|e| e.to_string())? {
            parameter_types.push(descr);
        }

        let return_type = get_next(&mut chars)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Missing return type".to_string())?;
        Ok(Self::new(parameter_types, return_type))
    }
}

impl Display for DescriptorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DescriptorError::UnexpectedEndOfInput => "Unexpected end of input".to_string(),
            DescriptorError::InvalidDescriptor => "Invalid descriptor".to_string(),
            DescriptorError::UnrecognizedTypeDescriptor => {
                "Unrecognized type descriptor".to_string()
            }
            DescriptorError::MissingSemicolonInClassNameDescriptor => {
                "Missing ';' in class name descriptor".to_string()
            }
            DescriptorError::TooManyDimensions => "Array has too many dimensions".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_boolean_type() {
        assert_eq!(str::parse::<TypeDescriptor>("Z"), Ok(Boolean));
    }
    #[test]
    fn should_parse_byte_type() {
        assert_eq!(str::parse::<TypeDescriptor>("B"), Ok(Byte));
    }
    #[test]
    fn should_parse_char_type() {
        assert_eq!(str::parse::<TypeDescriptor>("C"), Ok(Char));
    }
    #[test]
    fn should_parse_short_type() {
        assert_eq!(str::parse::<TypeDescriptor>("S"), Ok(Short));
    }
    #[test]
    fn should_parse_int_type() {
        assert_eq!(str::parse::<TypeDescriptor>("I"), Ok(Int));
    }
    #[test]
    fn should_parse_long_type() {
        assert_eq!(str::parse::<TypeDescriptor>("J"), Ok(Long));
    }
    #[test]
    fn should_parse_flot_type() {
        assert_eq!(str::parse::<TypeDescriptor>("F"), Ok(Float));
    }
    #[test]
    fn should_parse_double_type() {
        assert_eq!(str::parse::<TypeDescriptor>("D"), Ok(Double));
    }
    #[test]
    fn should_parse_1d_array_type() {
        assert_eq!(
            str::parse::<TypeDescriptor>("[I"),
            Ok(Array(Box::new(Int), 1))
        );
    }
    #[test]
    fn should_parse_2d_array_type() {
        assert_eq!(
            str::parse::<TypeDescriptor>("[[I"),
            Ok(Array(Box::new(Int), 2))
        );
    }
    #[test]
    fn should_parse_3d_array_type() {
        assert_eq!(
            str::parse::<TypeDescriptor>("[[[I"),
            Ok(Array(Box::new(Int), 3))
        );
    }
    #[test]
    fn should_parse_255d_array_type() {
        let mut type_descriptor = String::new();
        type_descriptor.push_str("[".repeat(255usize).as_str());
        type_descriptor.push_str("I");
        assert_eq!(
            str::parse::<TypeDescriptor>(type_descriptor.as_str()),
            Ok(Array(Box::new(Int), 255))
        );
    }
    #[test]
    fn should_parse_object_type() {
        assert_eq!(
            str::parse::<TypeDescriptor>("Ljava/lang/Object;"),
            Ok(Object("java/lang/Object".to_string()))
        );
    }

    #[test]
    fn should_parse_method_descriptor_with_primitives() {
        // int add(int a, int b)
        assert_eq!(
            MethodDescriptor::from_str("(II)I"),
            Ok(MethodDescriptor::new(vec![Int, Int], Int))
        );
    }
    #[test]
    fn should_parse_method_descriptor_with_string_array() {
        // void main(String[] args)
        assert_eq!(
            MethodDescriptor::from_str("([Ljava/lang/String;)V"),
            Ok(MethodDescriptor::new(
                vec![Array(Box::new(Object("java/lang/String".to_string())), 1)],
                Void
            ))
        );
    }
    #[test]
    fn should_parse_method_descriptor_with_arrays_of_primitives() {
        // int[] combineArrays(int[], int[])
        assert_eq!(
            MethodDescriptor::from_str("([I[I)[I"),
            Ok(MethodDescriptor::new(
                vec![Array(Box::new(Int), 1), Array(Box::new(Int), 1)],
                Array(Box::new(Int), 1)
            ))
        );
    }
    #[test]
    fn should_parse_method_descriptor_with_3d_arrays() {
        // int[][][] process(String[][][] data)
        assert_eq!(
            MethodDescriptor::from_str("([[[Ljava/lang/String;)[[[I"),
            Ok(MethodDescriptor::new(
                vec![Array(Box::new(Object("java/lang/String".to_string())), 3)],
                Array(Box::new(Int), 3)
            ))
        );
    }
    #[test]
    fn should_parse_method_descriptor_with_nested_object_types() {
        // boolean casTabAt(Node<K,V>[] tab, int i, Node<K,V> c, Node<K,V> v)
        assert_eq!(MethodDescriptor::from_str("([Ljava/util/concurrent/ConcurrentHashMap$Node;ILjava/util/concurrent/ConcurrentHashMap$Node;Ljava/util/concurrent/ConcurrentHashMap$Node;)Z"),
                   Ok(MethodDescriptor::new(vec![Array(Box::new(Object("java/util/concurrent/ConcurrentHashMap$Node".to_string())), 1), Int, Object("java/util/concurrent/ConcurrentHashMap$Node".to_string()), Object("java/util/concurrent/ConcurrentHashMap$Node".to_string())], Boolean)));
    }
    #[test]
    fn should_parse_method_descriptor_with_complex_generics() {
        // V merge(K, V, java.util.function.BiFunction<? super V, ? super V, ? extends V>)
        assert_eq!(MethodDescriptor::from_str("(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/lang/Object;"),
                   Ok(MethodDescriptor::new(vec![Object("java/lang/Object".to_string()), Object("java/lang/Object".to_string()), Object("java/util/function/BiFunction".to_string())], Object("java/lang/Object".to_string()))));
    }

    #[test]
    fn should_return_error_for_unsupported_type() {
        assert_eq!(
            str::parse::<TypeDescriptor>("X"),
            Err("Unrecognized type descriptor".to_string())
        );
    }
    #[test]
    fn should_return_error_for_array_without_type() {
        assert_eq!(
            str::parse::<TypeDescriptor>("["),
            Err("Unexpected end of input".to_string())
        );
    }
    #[test]
    fn should_return_error_for_object_without_closing_symbol() {
        assert_eq!(
            str::parse::<TypeDescriptor>("["),
            Err("Unexpected end of input".to_string())
        );
    }
    #[test]
    fn should_return_error_for_array_with_too_many_dimensions() {
        let mut type_descriptor = String::new();
        type_descriptor.push_str("[".repeat(256usize).as_str());
        type_descriptor.push_str("I");
        assert_eq!(
            str::parse::<TypeDescriptor>(type_descriptor.as_str()),
            Err("Array has too many dimensions".to_string())
        );
    }
}
