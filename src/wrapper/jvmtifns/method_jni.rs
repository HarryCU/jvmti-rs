use crate::{errors::*, objects::*, JVMTIEnv, Desc, Transform};
use jni::strings::JNIString;

impl<'a> JVMTIEnv<'a> {
    pub fn get_method_id<K, M, V>(&self, jni: &jni::JNIEnv<'a>, class: K, name: M, sig: V) -> Result<JMethodID>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        let klass: JClass = class.transform(jni)?;
        (klass, name, sig).lookup(jni)
    }

    pub fn get_static_method_id<K, M, V>(&self, jni: &jni::JNIEnv<'a>, class: K, name: M, sig: V) -> Result<JStaticMethodID>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        let klass: JClass = class.transform(jni)?;
        (klass, name, sig).lookup(jni)
    }
}
