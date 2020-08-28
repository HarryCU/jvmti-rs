use crate::sys::jmethodID;
use crate::{errors::*, objects::{JMethodID, JStaticMethodID}, Transform, AdapterTransform};
use jni::JNIEnv;

impl<'a> Transform<'a, jmethodID> for JMethodID<'a> {
    fn transform(self, _: &JNIEnv<'a>) -> Result<jmethodID> {
        Ok(self.into_inner())
    }
}

impl<'a> Transform<'a, jmethodID> for JStaticMethodID<'a> {
    fn transform(self, _: &JNIEnv<'a>) -> Result<jmethodID> {
        Ok(self.into_inner())
    }
}

impl<'a> AdapterTransform<jmethodID> for JMethodID<'a> {
    fn transform(self) -> jmethodID {
        self.into_inner()
    }
}

impl<'a> AdapterTransform<jmethodID> for JStaticMethodID<'a> {
    fn transform(self) -> jmethodID {
        self.into_inner()
    }
}

