use crate::sys::jmethodID;
use crate::{errors::*, objects::{JMethodID, JStaticMethodID}, Transform};
use crate::JVMTIEnv;

impl<'a> Transform<'a, jmethodID> for JMethodID<'a> {
    fn transform(self, _: &JVMTIEnv<'a>) -> Result<jmethodID> {
        Ok(self.into_inner())
    }
}

impl<'a> Transform<'a, jmethodID> for JStaticMethodID<'a> {
    fn transform(self, _: &JVMTIEnv<'a>) -> Result<jmethodID> {
        Ok(self.into_inner())
    }
}

