use crate::*;
use crate::{objects::*, errors::*};
use jni::strings::JNIString;
use jni::JNIEnv;

impl<'a, 'b> Desc<'a, JClass<'a>> for &'b str {
    fn lookup(self, jni: &JNIEnv<'a>) -> Result<JClass<'a>> {
        let name: JNIString = self.into();
        name.lookup(jni)
    }
}

impl<'a> Desc<'a, JClass<'a>> for JNIString {
    fn lookup(self, jni: &JNIEnv<'a>) -> Result<JClass<'a>> {
        jni.find_class(self)
            .map_err(jni_lookup_error)
    }
}

impl<'a, 'b> Desc<'a, JClass<'a>> for JObject<'b> {
    fn lookup(self, jni: &JNIEnv<'a>) -> Result<JClass<'a>> {
        jni.get_object_class(self)
            .map_err(jni_lookup_error)
    }
}