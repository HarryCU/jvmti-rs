use crate::*;
use crate::{objects::*, errors::*};
use jni::strings::JNIString;
use jni::JNIEnv;

impl<'a, U, V> Desc<'a, JFieldID<'a>> for (JClass<'a>, U, V)
    where
        U: Into<JNIString>,
        V: Into<JNIString>, {
    fn lookup(self, jni: &JNIEnv<'a>) -> Result<JFieldID<'a>> {
        jni.get_field_id(self.0, self.1, self.2)
            .map_err(jni_lookup_error)
    }
}

impl<'a, U, V> Desc<'a, JStaticFieldID<'a>> for (JClass<'a>, U, V)
    where
        U: Into<JNIString>,
        V: Into<JNIString>, {
    fn lookup(self, jni: &JNIEnv<'a>) -> Result<JStaticFieldID<'a>> {
        jni.get_static_field_id(self.0, self.1, self.2)
            .map_err(jni_lookup_error)
    }
}
