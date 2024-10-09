use crate::method_area::method_area::with_method_area;

pub(crate) struct InstanceChecker {}

impl InstanceChecker {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl InstanceChecker {
    pub fn checkcast(
        &self,
        class_cast_from: &str,
        class_cast_to: &str,
    ) -> crate::error::Result<bool> {
        if let Some(base_of) = Self::is_base_of(class_cast_to, class_cast_from) {
            return Ok(base_of);
        }

        if let Some(implements) = Self::is_implements(class_cast_to, class_cast_from) {
            return Ok(implements);
        }

        Ok(false)
    }

    fn is_base_of(base: &str, child: &str) -> Option<bool> {
        if base == child {
            return Some(true);
        }

        let class = with_method_area(|method_area| method_area.get(child)).ok()?;
        let class_name = class.parent().clone()?;

        Self::is_base_of(base, &class_name)
    }

    fn is_implements(interface: &str, implementor: &str) -> Option<bool> {
        let class = with_method_area(|method_area| method_area.get(interface)).ok()?;
        if !class.is_interface() {
            return None;
        }
        let class_implementor =
            with_method_area(|method_area| method_area.get(implementor)).ok()?;

        if class_implementor.interfaces().contains(interface) {
            return Some(true);
        }

        let class_name = class_implementor.parent().clone()?;

        Self::is_implements(interface, &class_name)
    }
}
