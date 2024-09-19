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
            .get_method_by_name_signature(main_class_name, "main:([Ljava/lang/String;)V")?;

        let mut engine = Engine::new(&self.class_loader.method_area());

        for (class_name, java_class) in self.class_loader.method_area().loaded_classes.iter() {
            if let Some(java_method) = java_class.methods.method_by_signature.get("<clinit>:()V") {
                println!(
                    "About to initialize java class {class_name} java_method={java_method:?}"
                );
                engine.execute(java_method)?; //todo implement multiclass correct initialization order
            }
        }

        engine.execute(main_method)
    }
}
