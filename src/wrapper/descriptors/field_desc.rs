use crate::*;
use crate::{objects::*, errors::*};
use jni::strings::JNIString;

impl<'a, U, V> Desc<'a, JFieldID<'a>> for (JClass<'a>, U, V)
    where
        U: Into<JNIString>,
        V: Into<JNIString>, {
    fn lookup(self, env: &JVMTIEnv<'a>) -> Result<JFieldID<'a>> {
        let jni = env.get_jni()?;
        jni.get_field_id(self.0, self.1, self.2)
            .map_err(jni_lookup_error)
    }
}

impl<'a, U, V> Desc<'a, JStaticFieldID<'a>> for (JClass<'a>, U, V)
    where
        U: Into<JNIString>,
        V: Into<JNIString>, {
    fn lookup(self, env: &JVMTIEnv<'a>) -> Result<JStaticFieldID<'a>> {
        let jni = env.get_jni()?;
        jni.get_static_field_id(self.0, self.1, self.2)
            .map_err(jni_lookup_error)
    }
}
