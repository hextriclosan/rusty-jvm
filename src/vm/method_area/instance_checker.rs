use crate::vm::error::Result;
use crate::vm::method_area::method_area::with_method_area;

pub(crate) struct InstanceChecker {}

impl InstanceChecker {
    pub fn checkcast(class_cast_from: &str, class_cast_to: &str) -> Result<bool> {
        let (class_cast_from, class_cast_to) =
            Self::try_unwrap_arrays(class_cast_from, class_cast_to);

        if let Some(base_of) = Self::is_base_of(class_cast_to, class_cast_from) {
            return Ok(base_of);
        }

        let class = with_method_area(|method_area| method_area.get(class_cast_to))?;
        if class.is_interface() {
            if let Some(implements) = Self::implements(class_cast_to, class_cast_from) {
                return Ok(implements);
            }
        }

        Ok(false)
    }

    fn try_unwrap_arrays<'a>(first: &'a str, second: &'a str) -> (&'a str, &'a str) {
        fn unwrap_descriptor(descr: &str) -> &str {
            if descr.starts_with('L') {
                &descr[1..descr.len() - 1]
            } else {
                descr
            }
        }

        if first.starts_with('[') && second.starts_with('[') {
            Self::try_unwrap_arrays(&first[1..], &second[1..])
        } else {
            let first = unwrap_descriptor(first);
            let second = unwrap_descriptor(second);
            (first, second)
        }
    }

    fn is_base_of(base: &str, child: &str) -> Option<bool> {
        if base == child {
            return Some(true);
        }

        let class = with_method_area(|method_area| method_area.get(child)).ok()?;
        let class_name = class.parent().clone()?;

        Self::is_base_of(base, &class_name)
    }

    fn implements(interface: &str, implementor: &str) -> Option<bool> {
        let class_implementor =
            with_method_area(|method_area| method_area.get(implementor)).ok()?;

        let interfaces = class_implementor.interfaces();
        for interface_name in interfaces.iter() {
            if interface_name == interface {
                return Some(true);
            }

            if let Some(implements) = Self::implements(interface, interface_name) {
                return Some(implements);
            }
        }

        let class_name = class_implementor.parent().clone()?;

        Self::implements(interface, &class_name)
    }
}
