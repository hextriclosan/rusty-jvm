use crate::class_loader::ClassLoader;
use crate::execution_engine::engine::Engine;

#[derive(Debug)]
pub struct VM {
    class_loader: ClassLoader,
}

impl VM {
    pub fn new(class_file_name: &str, std_dir: &str) -> crate::error::Result<Self> {
        let class_loader = ClassLoader::new(class_file_name, std_dir)?;
        Ok(Self { class_loader })
    }

    pub fn run(&self) -> crate::error::Result<Option<i32>> {
        let main_class_name = self.class_loader.method_area().main_class_name.as_str();

        let main_method = self
            .class_loader
            .method_area()
            .get_method_by_name_signature(main_class_name, "main:([Ljava/lang/String;)V")?;

        let mut engine = Engine::new(&self.class_loader.method_area());
        engine.execute(main_class_name, main_method)
    }
}
