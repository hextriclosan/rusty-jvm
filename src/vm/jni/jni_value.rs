use crate::vm::helper::{i64_to_vec, vec_to_i64};
use crate::vm::jni::handles;
use crate::vm::stack::stack_value::StackValue;
use jni_sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jobject, jshort};

pub trait JNIValue: Default {
    fn from_vec(v: &[i32]) -> Self;
    fn to_vec(&self) -> Option<Vec<i32>>;
}

impl JNIValue for jobject {
    fn from_vec(v: &[i32]) -> Self {
        handles::new_local(v[0])
    }

    fn to_vec(&self) -> Option<Vec<i32>> {
        handles::resolve(*self).map(|target| vec![target])
    }
}

impl JNIValue for jboolean {
    fn from_vec(v: &[i32]) -> Self {
        v[0] != 0
    }

    fn to_vec(&self) -> Option<Vec<i32>> {
        Some(vec![if *self { 1 } else { 0 }])
    }
}

impl JNIValue for jbyte {
    fn from_vec(v: &[i32]) -> Self {
        v[0] as jbyte
    }

    fn to_vec(&self) -> Option<Vec<i32>> {
        Some(vec![*self as i32])
    }
}

impl JNIValue for jchar {
    fn from_vec(v: &[i32]) -> Self {
        v[0] as jchar
    }

    fn to_vec(&self) -> Option<Vec<i32>> {
        Some(vec![*self as i32])
    }
}

impl JNIValue for jshort {
    fn from_vec(v: &[i32]) -> Self {
        v[0] as jshort
    }

    fn to_vec(&self) -> Option<Vec<i32>> {
        Some(vec![*self as i32])
    }
}

impl JNIValue for jint {
    fn from_vec(v: &[i32]) -> Self {
        v[0] as jint
    }

    fn to_vec(&self) -> Option<Vec<i32>> {
        Some(vec![(*self)])
    }
}

impl JNIValue for jlong {
    fn from_vec(v: &[i32]) -> Self {
        vec_to_i64(v)
    }

    fn to_vec(&self) -> Option<Vec<i32>> {
        Some(i64_to_vec(*self))
    }
}

impl JNIValue for jfloat {
    fn from_vec(v: &[i32]) -> Self {
        let value: i32 = JNIValue::from_vec(v);
        f32::from_bits(value as u32)
    }

    fn to_vec(&self) -> Option<Vec<i32>> {
        Some(vec![self.to_bits() as i32])
    }
}

impl JNIValue for jdouble {
    fn from_vec(v: &[i32]) -> Self {
        let value: i64 = JNIValue::from_vec(v);
        f64::from_bits(value as u64)
    }

    fn to_vec(&self) -> Option<Vec<i32>> {
        Some(StackValue::to_vec(&(self.to_bits() as i64)))
    }
}

impl JNIValue for () {
    fn from_vec(_v: &[i32]) -> Self {}

    fn to_vec(&self) -> Option<Vec<i32>> {
        Some(vec![])
    }
}
