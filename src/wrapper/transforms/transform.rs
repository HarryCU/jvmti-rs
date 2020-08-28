use crate::errors::*;
use crate::JVMTIEnv;

pub trait Transform<'a, T> {
    fn transform(self, _: &JVMTIEnv<'a>) -> Result<T>;
}