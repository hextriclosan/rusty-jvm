use crate::error::Error;
use crate::execution_engine::engine::Engine;
use crate::heap::heap::Heap;
use crate::method_area::method_area::MethodArea;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct VM {
    method_area: Rc<RefCell<MethodArea>>,
    heap: Rc<RefCell<Heap>>,
}

impl VM {
    const ENTRY_POINT: &'static str = "main:([Ljava/lang/String;)V";

    pub fn new(std_dir: &str) -> Self {
        let heap = Rc::new(RefCell::new(Heap::new()));
        Self {
            method_area: MethodArea::new(std_dir, Rc::clone(&heap)),
            heap,
        }
    }

    pub fn run(&mut self, main_class_name: &str) -> crate::error::Result<Option<Vec<i32>>> {
        let internal_name = &main_class_name.replace('.', "/");
        let java_class = self.method_area.borrow().get(internal_name)?;

        let java_method = java_class
            .methods
            .method_by_signature
            .get(Self::ENTRY_POINT)
            .ok_or(Error::new_execution(
                format!("main method not found in {main_class_name}").as_str(),
            ))?;

        let mut engine = Engine::new(Rc::clone(&self.method_area), Rc::clone(&self.heap));

        engine.execute(java_method)
    }
}
