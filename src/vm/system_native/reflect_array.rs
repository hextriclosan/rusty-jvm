use crate::vm::error::Result;
use crate::vm::exception::pending_helpers::{
    set_pending_array_index_out_of_bounds_exception, set_pending_illegal_argument_exception,
    set_pending_null_pointer_exception,
};
use crate::vm::execution_engine::executor::Executor;
use crate::vm::heap::heap::HEAP;
use crate::vm::helper::{decorate, klass};
use crate::vm::jni::java_thread::JavaThread;
use crate::vm::method_area::instance_checker::InstanceChecker;
use crate::vm::stack::rooted_slots::RootedSlots;
use crate::vm::stack::slot::Slot;
use crate::vm::stack::stack_value::{StackValue, StackValueKind};

#[derive(Clone, Copy)]
enum Primitive {
    Boolean(bool),
    Byte(i8),
    Char(u16),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
}

impl Primitive {
    fn from_array(component: &str, value: &[i32]) -> Option<Self> {
        Some(match component {
            "Z" => Self::Boolean(value[0] != 0),
            "B" => Self::Byte(value[0] as i8),
            "C" => Self::Char(value[0] as u16),
            "S" => Self::Short(value[0] as i16),
            "I" => Self::Int(value[0]),
            "J" => Self::Long(i64::from_vec(value)),
            "F" => Self::Float(f32::from_vec(value)),
            "D" => Self::Double(f64::from_vec(value)),
            _ => return None,
        })
    }

    fn from_wrapper(reference: i32, class_name: &str) -> Result<Option<Self>> {
        let value = || HEAP.get_object_field_value(reference, class_name, "value");
        Ok(Some(match class_name {
            "java/lang/Boolean" => Self::Boolean(value()?[0] != 0),
            "java/lang/Byte" => Self::Byte(value()?[0] as i8),
            "java/lang/Character" => Self::Char(value()?[0] as u16),
            "java/lang/Short" => Self::Short(value()?[0] as i16),
            "java/lang/Integer" => Self::Int(value()?[0]),
            "java/lang/Long" => Self::Long(i64::from_vec(&value()?)),
            "java/lang/Float" => Self::Float(f32::from_vec(&value()?)),
            "java/lang/Double" => Self::Double(f64::from_vec(&value()?)),
            _ => return Ok(None),
        }))
    }

    fn widen_to(self, component: &str) -> Option<Self> {
        Some(match (self, component) {
            (Self::Boolean(value), "Z") => Self::Boolean(value),
            (Self::Byte(value), "B") => Self::Byte(value),
            (Self::Byte(value), "S") => Self::Short(value.into()),
            (Self::Byte(value), "I") => Self::Int(value.into()),
            (Self::Byte(value), "J") => Self::Long(value.into()),
            (Self::Byte(value), "F") => Self::Float(value.into()),
            (Self::Byte(value), "D") => Self::Double(value.into()),
            (Self::Char(value), "C") => Self::Char(value),
            (Self::Char(value), "I") => Self::Int(value.into()),
            (Self::Char(value), "J") => Self::Long(value.into()),
            (Self::Char(value), "F") => Self::Float(value.into()),
            (Self::Char(value), "D") => Self::Double(value.into()),
            (Self::Short(value), "S") => Self::Short(value),
            (Self::Short(value), "I") => Self::Int(value.into()),
            (Self::Short(value), "J") => Self::Long(value.into()),
            (Self::Short(value), "F") => Self::Float(value.into()),
            (Self::Short(value), "D") => Self::Double(value.into()),
            (Self::Int(value), "I") => Self::Int(value),
            (Self::Int(value), "J") => Self::Long(value.into()),
            (Self::Int(value), "F") => Self::Float(value as f32),
            (Self::Int(value), "D") => Self::Double(value as f64),
            (Self::Long(value), "J") => Self::Long(value),
            (Self::Long(value), "F") => Self::Float(value as f32),
            (Self::Long(value), "D") => Self::Double(value as f64),
            (Self::Float(value), "F") => Self::Float(value),
            (Self::Float(value), "D") => Self::Double(value.into()),
            (Self::Double(value), "D") => Self::Double(value),
            _ => return None,
        })
    }

    fn stack_value(self) -> StackValueKind {
        match self {
            Self::Boolean(value) => StackValueKind::I32(i32::from(value)),
            Self::Byte(value) => StackValueKind::I32(value.into()),
            Self::Char(value) => StackValueKind::I32(value.into()),
            Self::Short(value) => StackValueKind::I32(value.into()),
            Self::Int(value) => StackValueKind::I32(value),
            Self::Long(value) => StackValueKind::I64(value),
            Self::Float(value) => StackValueKind::F32(value),
            Self::Double(value) => StackValueKind::F64(value),
        }
    }

    fn raw(self) -> Vec<i32> {
        match self {
            Self::Boolean(value) => vec![i32::from(value)],
            Self::Byte(value) => vec![value.into()],
            Self::Char(value) => vec![value.into()],
            Self::Short(value) => vec![value.into()],
            Self::Int(value) => vec![value],
            Self::Long(value) => value.to_vec(),
            Self::Float(value) => value.to_vec(),
            Self::Double(value) => value.to_vec(),
        }
    }

    fn boxed(self) -> Result<i32> {
        let (class_name, signature) = match self {
            Self::Boolean(_) => ("java/lang/Boolean", "valueOf:(Z)Ljava/lang/Boolean;"),
            Self::Byte(_) => ("java/lang/Byte", "valueOf:(B)Ljava/lang/Byte;"),
            Self::Char(_) => ("java/lang/Character", "valueOf:(C)Ljava/lang/Character;"),
            Self::Short(_) => ("java/lang/Short", "valueOf:(S)Ljava/lang/Short;"),
            Self::Int(_) => ("java/lang/Integer", "valueOf:(I)Ljava/lang/Integer;"),
            Self::Long(_) => ("java/lang/Long", "valueOf:(J)Ljava/lang/Long;"),
            Self::Float(_) => ("java/lang/Float", "valueOf:(F)Ljava/lang/Float;"),
            Self::Double(_) => ("java/lang/Double", "valueOf:(D)Ljava/lang/Double;"),
        };
        let result = Executor::invoke_static_method(class_name, signature, &[self.stack_value()])?;
        Ok(result[0].value())
    }
}

fn array_info(array_ref: i32) -> Result<Option<(String, i32)>> {
    if array_ref == 0 {
        set_pending_null_pointer_exception()?;
        return Ok(None);
    }
    let type_name = HEAP.get_instance_name(array_ref)?;
    if !type_name.starts_with('[') {
        set_pending_illegal_argument_exception("argument is not an array")?;
        return Ok(None);
    }
    let length = HEAP.get_array_len(array_ref)?;
    Ok(Some((type_name, length)))
}

fn check_index(index: i32, length: i32) -> Result<bool> {
    if index < 0 || index >= length {
        set_pending_array_index_out_of_bounds_exception(&format!(
            "Index {index} out of bounds for length {length}"
        ))?;
        return Ok(false);
    }
    Ok(true)
}

fn illegal_argument<T: Default>(message: &str) -> Result<T> {
    set_pending_illegal_argument_exception(message)?;
    Ok(T::default())
}

/// `java.lang.reflect.Array.newArray(Ljava/lang/Class;I)Ljava/lang/Object;`
pub(crate) fn new_array(component_type_clazz_ref: i32, length: i32) -> Result<i32> {
    let _component_type = JavaThread::hold_temp_roots(vec![Slot::Ref(component_type_clazz_ref)]);
    let klass = klass(component_type_clazz_ref)?;
    let class_name = klass.this_class_name().to_owned();
    let decorated = decorate(class_name);
    let decorated_array = format!("[{decorated}");
    HEAP.create_array(&decorated_array, length, &mut RootedSlots::new())
}

/// `java.lang.reflect.Array.getLength(Ljava/lang/Object;)I`
pub(crate) fn get_length(array_ref: i32) -> Result<i32> {
    let _array = JavaThread::hold_temp_roots(vec![Slot::Ref(array_ref)]);
    Ok(array_info(array_ref)?.map_or(0, |(_, length)| length))
}

/// `java.lang.reflect.Array.get(Ljava/lang/Object;I)Ljava/lang/Object;`
pub(crate) fn get(array_ref: i32, index: i32) -> Result<i32> {
    let _array = JavaThread::hold_temp_roots(vec![Slot::Ref(array_ref)]);
    let Some((type_name, length)) = array_info(array_ref)? else {
        return Ok(0);
    };
    if !check_index(index, length)? {
        return Ok(0);
    }

    let value = HEAP.get_array_value(array_ref, index)?;
    let component = &type_name[1..];
    if component.starts_with('L') || component.starts_with('[') {
        return Ok(value[0]);
    }

    let Some(value) = Primitive::from_array(component, &value) else {
        return Err(crate::vm::gc::budget::AllocError::InvalidLayout.into());
    };
    value.boxed()
}

/// `java.lang.reflect.Array.getInt(Ljava/lang/Object;I)I`
pub(crate) fn get_int(array_ref: i32, index: i32) -> Result<i32> {
    let _array = JavaThread::hold_temp_roots(vec![Slot::Ref(array_ref)]);
    let Some((type_name, length)) = array_info(array_ref)? else {
        return Ok(0);
    };
    if !check_index(index, length)? {
        return Ok(0);
    }

    let component = &type_name[1..];
    match Primitive::from_array(component, &HEAP.get_array_value(array_ref, index)?) {
        Some(Primitive::Byte(value)) => Ok(value.into()),
        Some(Primitive::Char(value)) => Ok(value.into()),
        Some(Primitive::Short(value)) => Ok(value.into()),
        Some(Primitive::Int(value)) => Ok(value),
        _ => illegal_argument("array element cannot be converted to int"),
    }
}

/// `java.lang.reflect.Array.set(Ljava/lang/Object;ILjava/lang/Object;)V`
pub(crate) fn set(array_ref: i32, index: i32, value_ref: i32) -> Result<()> {
    let _roots = JavaThread::hold_temp_roots(vec![Slot::Ref(array_ref), Slot::Ref(value_ref)]);
    let Some((type_name, length)) = array_info(array_ref)? else {
        return Ok(());
    };
    if !check_index(index, length)? {
        return Ok(());
    }

    let component = &type_name[1..];
    if component.starts_with('L') || component.starts_with('[') {
        if value_ref != 0 {
            let value_name = HEAP.get_instance_name(value_ref)?;
            let target_name = if component.starts_with('L') {
                &component[1..component.len() - 1]
            } else {
                component
            };
            if !InstanceChecker::checkcast(&value_name, target_name)? {
                return illegal_argument("array element type mismatch");
            }
        }
        return HEAP.set_array_value(array_ref, index, vec![value_ref]);
    }

    if value_ref == 0 {
        return illegal_argument("null cannot be assigned to a primitive array");
    }
    let value_name = HEAP.get_instance_name(value_ref)?;
    let Some(value) = Primitive::from_wrapper(value_ref, &value_name)? else {
        return illegal_argument("value cannot be converted to the array component type");
    };
    let Some(value) = value.widen_to(component) else {
        return illegal_argument("value cannot be converted to the array component type");
    };
    HEAP.set_array_value(array_ref, index, value.raw())
}

/// `java.lang.reflect.Array.setInt(Ljava/lang/Object;II)V`
pub(crate) fn set_int(array_ref: i32, index: i32, value: i32) -> Result<()> {
    let _array = JavaThread::hold_temp_roots(vec![Slot::Ref(array_ref)]);
    let Some((type_name, length)) = array_info(array_ref)? else {
        return Ok(());
    };
    if !check_index(index, length)? {
        return Ok(());
    }
    let Some(value) = Primitive::Int(value).widen_to(&type_name[1..]) else {
        return illegal_argument("int cannot be converted to the array component type");
    };
    HEAP.set_array_value(array_ref, index, value.raw())
}
