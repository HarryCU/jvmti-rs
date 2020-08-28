use crate::sys::jfieldID;
use crate::{errors::*, objects::{JStaticFieldID, JFieldID}, Transform, AdapterTransform};
use jni::JNIEnv;

impl<'a> Transform<'a, jfieldID> for JFieldID<'a> {
    fn transform(self, _: &JNIEnv<'a>) -> Result<jfieldID> {
        Ok(self.into_inner())
    }
}

impl<'a> Transform<'a, jfieldID> for JStaticFieldID<'a> {
    fn transform(self, _: &JNIEnv<'a>) -> Result<jfieldID> {
        Ok(self.into_inner())
    }
}

impl<'a> AdapterTransform<jfieldID> for JFieldID<'a> {
    fn transform(self) -> jfieldID {
        self.into_inner()
    }
}

impl<'a> AdapterTransform<jfieldID> for JStaticFieldID<'a> {
    fn transform(self) -> jfieldID {
        self.into_inner()
    }
}

