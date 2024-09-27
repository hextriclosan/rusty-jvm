use crate::error::Error;
use crate::execution_engine::engine::Engine;
use crate::method_area::method_area::MethodArea;

#[derive(Debug)]
pub struct VM {
    method_area: MethodArea,
}

impl VM {
    const ENTRY_POINT: &'static str = "main:([Ljava/lang/String;)V";

    pub fn new(std_dir: &str) -> Self {
        Self {
            method_area: MethodArea::new(std_dir),
        }
    }

    pub fn run(&mut self, main_class_name: &str) -> crate::error::Result<Option<Vec<i32>>> {
        let internal_name = &main_class_name.replace('.', "/");
        let java_class = self.method_area.get(internal_name)?;

        let java_method = java_class
            .methods
            .method_by_signature
            .get(Self::ENTRY_POINT)
            .ok_or(Error::new_execution(
                format!("main method not found in {main_class_name}").as_str(),
            ))?;

        let mut engine = Engine::new(&self.method_area);

        engine.execute(java_method)
    }
}
