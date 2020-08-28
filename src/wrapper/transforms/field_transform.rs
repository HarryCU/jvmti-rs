use crate::sys::jfieldID;
use crate::{errors::*, objects::{JStaticFieldID, JFieldID}, Transform};
use crate::JVMTIEnv;

impl<'a> Transform<'a, jfieldID> for JFieldID<'a> {
    fn transform(self, _: &JVMTIEnv<'a>) -> Result<jfieldID> {
        Ok(self.into_inner())
    }
}

impl<'a> Transform<'a, jfieldID> for JStaticFieldID<'a> {
    fn transform(self, _: &JVMTIEnv<'a>) -> Result<jfieldID> {
        Ok(self.into_inner())
    }
}

