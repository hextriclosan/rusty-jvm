use crate::class_loader::ClassLoader;
use crate::execution_engine::engine::Engine;

#[derive(Debug)]
pub struct VM {
    class_loader: ClassLoader,
}

impl VM {
    pub fn new(class_file_names: Vec<&str>, std_dir: &str) -> crate::error::Result<Self> {
        let class_loader = ClassLoader::new(class_file_names, std_dir)?;
        Ok(Self { class_loader })
    }

    pub fn run(&self, main_class_name: &str) -> crate::error::Result<Option<Vec<i32>>> {
        let main_method = self
            .class_loader
            .method_area()
            .get_method_by_name_signature(
                &main_class_name.replace('.', "/"),
                "main:([Ljava/lang/String;)V",
            )?;

        let mut engine = Engine::new(&self.class_loader.method_area());

        engine.execute(main_method)
    }
}
