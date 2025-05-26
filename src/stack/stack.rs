#[derive(Clone, Debug)]
pub(crate) struct Stack<T> {
    max_size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn with_capacity(max_size: usize) -> Self {
        Stack {
            max_size,
            data: Vec::with_capacity(max_size),
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), String> {
        if self.data.len() >= self.max_size {
            return Err("Exceeded max stack size".to_string());
        }
        self.data.push(item);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut stack = Stack::with_capacity(3);
        assert_eq!(stack.push(1), Ok(()));
        assert_eq!(stack.push(2), Ok(()));
        assert_eq!(stack.push(3), Ok(()));
        assert_eq!(stack.push(4), Err("Exceeded max stack size".to_string()));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }
}
