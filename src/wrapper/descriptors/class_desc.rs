use crate::*;
use crate::{objects::*, errors::*};
use jni::strings::JNIString;

impl<'a, 'b> Desc<'a, JClass<'a>> for &'b str {
    fn lookup(self, env: &JVMTIEnv<'a>) -> Result<JClass<'a>> {
        let name: JNIString = self.into();
        name.lookup(env)
    }
}

impl<'a> Desc<'a, JClass<'a>> for JNIString {
    fn lookup(self, env: &JVMTIEnv<'a>) -> Result<JClass<'a>> {
        let jni = env.get_jni()?;
        jni.find_class(self)
            .map_err(jni_lookup_error)
    }
}

impl<'a, 'b> Desc<'a, JClass<'a>> for JObject<'b> {
    fn lookup(self, env: &JVMTIEnv<'a>) -> Result<JClass<'a>> {
        let jni = env.get_jni()?;
        jni.get_object_class(self)
            .map_err(jni_lookup_error)
    }
}