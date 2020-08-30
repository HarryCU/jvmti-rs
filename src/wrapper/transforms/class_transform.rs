use crate::{Transform, Desc};
use jni::strings::JNIString;
use crate::{errors::*, objects::JClass};
use jni::JNIEnv;

impl<'a> Transform<'a, JClass<'a>> for &'a str {
    fn transform(self, jni: &JNIEnv<'a>) -> Result<JClass<'a>> {
        jni.find_class(self)
            .map_err(jni_lookup_error)
    }
}

impl<'a> Transform<'a, JClass<'a>> for JNIString {
    fn transform(self, jni: &JNIEnv<'a>) -> Result<JClass<'a>> {
        self.lookup(jni)
    }
}

impl<'a> Transform<'a, JClass<'a>> for JClass<'a> {
    fn transform(self, _: &JNIEnv<'a>) -> Result<JClass<'a>> {
        Ok(self)
    }
}