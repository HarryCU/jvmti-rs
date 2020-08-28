use crate::errors::*;
use jni::JNIEnv;

pub trait Transform<'a, T> {
    fn transform(self, _: &JNIEnv<'a>) -> Result<T>;
}

pub trait AdapterTransform<T> {
    fn transform(self) -> T;
}