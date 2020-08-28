use crate::{Transform, JVMTIEnv, Desc};
use jni::strings::JNIString;
use crate::{errors::*, objects::JClass};

impl<'a, 'b> Transform<'a, JClass<'a>> for &'b str {
    fn transform(self, env: &JVMTIEnv<'a>) -> Result<JClass<'a>> {
        let jni = env.get_jni()?;
        jni.find_class(self)
            .map_err(jni_lookup_error)
    }
}

impl<'a> Transform<'a, JClass<'a>> for JNIString {
    fn transform(self, env: &JVMTIEnv<'a>) -> Result<JClass<'a>> {
        self.lookup(env)
    }
}

impl<'a> Transform<'a, JClass<'a>> for JClass<'a> {
    fn transform(self, _: &JVMTIEnv<'a>) -> Result<JClass<'a>> {
        Ok(self)
    }
}