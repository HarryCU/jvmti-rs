use crate::*;
use crate::{objects::*, errors::*};
use jni::strings::JNIString;

impl<'a, U, V> Desc<'a, JMethodID<'a>> for (JClass<'a>, U, V)
    where
        U: Into<JNIString>,
        V: Into<JNIString>, {
    fn lookup(self, env: &JVMTIEnv<'a>) -> Result<JMethodID<'a>> {
        let jni = env.get_jni()?;
        jni.get_method_id(self.0, self.1, self.2)
            .map_err(jni_lookup_error)
    }
}

impl<'a, U> Desc<'a, JMethodID<'a>> for (JClass<'a>, U)
    where
        U: Into<JNIString>, {
    fn lookup(self, env: &JVMTIEnv<'a>) -> Result<JMethodID<'a>> {
        (self.0, "<init>", self.1).lookup(env)
    }
}

impl<'a, U, V> Desc<'a, JStaticMethodID<'a>> for (JClass<'a>, U, V)
    where
        U: Into<JNIString>,
        V: Into<JNIString>, {
    fn lookup(self, env: &JVMTIEnv<'a>) -> Result<JStaticMethodID<'a>> {
        let jni = env.get_jni()?;
        jni.get_static_method_id(self.0, self.1, self.2)
            .map_err(jni_lookup_error)
    }
}
