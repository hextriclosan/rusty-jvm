use crate::error::Error;
use crate::execution_engine::engine::Engine;
use crate::execution_engine::string_pool_helper::StringPoolHelper;
use crate::heap::heap::with_heap_write_lock;
use crate::method_area::method_area::{with_method_area, MethodArea};

#[derive(Debug)]
pub struct VM {}

impl VM {
    const ENTRY_POINT: &'static str = "main:([Ljava/lang/String;)V";

    pub fn new(std_dir: &str) -> crate::error::Result<Self> {
        MethodArea::init(std_dir);

        Self::init()?;

        Ok(Self {})
    }

    fn init() -> crate::error::Result<()> {
        // explicit static initialization of java.lang.ref.Reference needed for creating JavaLangRefAccess instance
        // reading Reference.processPendingActive
        Self::read_static_field("java/lang/ref/Reference", "processPendingActive")?;

        // create primordial ThreadGroup and Thread
        let tg_obj_ref = Self::create_thread_group()?;
        let string_obj_ref = StringPoolHelper::get_string("system".to_string())?; // refactor candidate B: introduce and use here common string creator, not string pool one
        let _thread_obj_ref = Self::create_thread(tg_obj_ref, string_obj_ref)?;

        Self::invoke_void_void_static_method("java/lang/System", "initPhase1:()V")?;

        Ok(())
    }

    fn invoke_void_void_static_method(
        // refactor move me to utils
        class_name: &str,
        method_name: &str,
    ) -> crate::error::Result<()> {
        let java_class = with_method_area(|area| area.get(class_name))?;

        let java_method = java_class
            .methods
            .method_by_signature
            .get(method_name)
            .ok_or(Error::new_execution(
                format!("method {method_name} not found in {class_name}").as_str(),
            ))?;

        let mut engine = Engine::new();

        let _result = engine.execute(
            java_method.new_stack_frame()?,
            &format!("invoke {class_name}.{method_name}"),
        )?;
        Ok(())
    }

    fn read_static_field(class_name: &str, method_name: &str) -> crate::error::Result<Vec<i32>> {
        let field =
            with_method_area(|area| area.lookup_for_static_field(class_name, method_name))?;

        let raw_value = field.raw_value();
        Ok(raw_value)
    }

    // refactor candidate A: generalize object creation and constructor invocation
    fn create_thread_group() -> crate::error::Result<i32> {
        let tg_class_name = "java/lang/ThreadGroup".to_string();
        let instance_with_default_fields = with_method_area(|method_area| {
            method_area.create_instance_with_default_fields(&tg_class_name)
        });

        let tg_instance_ref =
            with_heap_write_lock(|heap| heap.create_instance(instance_with_default_fields));

        let full_signature = "<init>:()V";
        let rc = with_method_area(|method_area| method_area.get(tg_class_name.as_str()))?;
        let special_method = rc
            .methods
            .method_by_signature
            .get(full_signature)
            .ok_or_else(|| Error::new_constant_pool(&format!("Error getting JavaMethod by class name {tg_class_name} and full signature {full_signature} invoking special")))?;

        let mut engine = Engine::new();
        let mut stack_frame = special_method.new_stack_frame()?;
        stack_frame.set_local(0, tg_instance_ref);

        engine.execute(stack_frame, "invoking java/lang/ThreadGroup()")?;

        Ok(tg_instance_ref)
    }

    // refactor candidate A: generalize object creation and constructor invocation
    fn create_thread(tg_obj_ref: i32, string_obj_ref: i32) -> crate::error::Result<i32> {
        let thread_class_name = "java/lang/Thread".to_string();
        let instance_with_default_fields = with_method_area(|method_area| {
            method_area.create_instance_with_default_fields(&thread_class_name)
        });

        let thread_instance_ref =
            with_heap_write_lock(|heap| heap.create_instance(instance_with_default_fields));
        with_method_area(|area| area.set_system_thread_id(thread_instance_ref)); //save primordial thread id

        let full_signature = "<init>:(Ljava/lang/ThreadGroup;Ljava/lang/String;)V";
        let rc = with_method_area(|method_area| method_area.get(thread_class_name.as_str()))?;
        let special_method = rc
            .methods
            .method_by_signature
            .get(full_signature)
            .ok_or_else(|| Error::new_constant_pool(&format!("Error getting JavaMethod by class name {thread_class_name} and full signature {full_signature} invoking special")))?;

        let mut engine = Engine::new();
        let mut stack_frame = special_method.new_stack_frame()?;
        stack_frame.set_local(0, thread_instance_ref);
        stack_frame.set_local(1, tg_obj_ref);
        stack_frame.set_local(2, string_obj_ref);

        engine.execute(
            stack_frame,
            "invoking java/lang/Thread(ThreadGroup group, String name)",
        )?;

        Ok(thread_instance_ref)
    }

    pub fn run(&mut self, main_class_name: &str) -> crate::error::Result<Option<Vec<i32>>> {
        let internal_name = &main_class_name.replace('.', "/");
        let java_class = with_method_area(|area| area.get(internal_name))?;

        let java_method = java_class
            .methods
            .method_by_signature
            .get(Self::ENTRY_POINT)
            .ok_or(Error::new_execution(
                format!("main method not found in {main_class_name}").as_str(),
            ))?;

        let mut engine = Engine::new();

        engine.execute(
            java_method.new_stack_frame()?,
            "invoke main:([Ljava/lang/String;)V",
        )
    }
}
