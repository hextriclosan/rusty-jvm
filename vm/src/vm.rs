use crate::class_loader::ClassLoader;
use crate::execution_engine::engine::Engine;


#[derive(Debug)]
pub struct VM {
    class_loader: ClassLoader,
}

impl VM {
    pub fn new(class_file_name: &str) -> crate::error::Result<Self> {
        let class_loader = ClassLoader::new(class_file_name)?;
        Ok(Self { class_loader })
    }

    pub fn run(&self) -> crate::error::Result<Option<i32>> {
        let main_method = self.class_loader.method_area()
            .get_method_by_name_signature("main:([Ljava/lang/String;)V")?;

        let engine = Engine::new(&self.class_loader.method_area());
        engine.execute(main_method)
    }
}
