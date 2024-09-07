use crate::error::Error;

#[derive(Debug)]
pub(crate) struct Signature {
    #[allow(dead_code)]
    parameter_types: Vec<SignatureType>,
    #[allow(dead_code)]
    return_type: SignatureType,
    arg_num: usize, //todo: remove this workaround
}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum SignatureType {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Boolean,
    Void,
    Array(Box<SignatureType>),
    Object(String),
}

impl Signature {
    pub fn from_str(signature: &str) -> crate::error::Result<Self> {
        Self::count_symbols_between_parentheses(signature)
            .and_then(|arg_num| {
                Some(Self {
                    parameter_types: vec![],
                    return_type: SignatureType::Void,
                    arg_num,
                })
            })
            .ok_or(Error::new_constant_pool("Error parsing signature"))
    }

    pub fn get_arg_num(&self) -> usize {
        self.arg_num
    }

    fn count_symbols_between_parentheses(s: &str) -> Option<usize> {
        //todo: remove this workaround
        let result = s.replace("[", ""); // workaround for arrays
        let open_index = result.find('(')?;
        let close_index = result.find(')')?;

        if open_index < close_index {
            Some(close_index - open_index - 1)
        } else {
            None
        }
    }
}
