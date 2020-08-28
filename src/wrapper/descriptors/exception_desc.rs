use crate::*;
use crate::{objects::*, errors::*};
use jni::strings::JNIString;
use jni::errors::Exception;
use jni::JNIEnv;

const DEFAULT_EXCEPTION_CLASS: &str = "java/lang/RuntimeException";

impl<'a, M> Desc<'a, JThrowable<'a>> for (JClass<'a>, M)
    where
        M: Into<JNIString>, {
    fn lookup(self, jni: &JNIEnv<'a>) -> Result<JThrowable<'a>> {
        let jmsg: JObject = jni.new_string(self.1)
            .map_err(jni_lookup_error)?
            .into();
        let obj: JThrowable = jni
            .new_object(self.0, "(Ljava/lang/String;)V", &[JValue::from(jmsg)])
            .map_err(jni_lookup_error)?
            .into();
        Ok(obj)
    }
}

impl<'a> Desc<'a, JThrowable<'a>> for Exception {
    fn lookup(self, jni: &JNIEnv<'a>) -> Result<JThrowable<'a>> {
        let jni_class: JNIString = self.class.into();
        let class: JClass = jni_class.lookup(jni)?;
        (class, self.msg).lookup(jni)
    }
}

impl<'a, 'b> Desc<'a, JThrowable<'a>> for &'b str {
    //noinspection DuplicatedCode
    fn lookup(self, jni: &JNIEnv<'a>) -> Result<JThrowable<'a>> {
        let class: JClass = DEFAULT_EXCEPTION_CLASS.lookup(jni)?;
        (class, self).lookup(jni)
    }
}

impl<'a> Desc<'a, JThrowable<'a>> for String {
    //noinspection DuplicatedCode
    fn lookup(self, jni: &JNIEnv<'a>) -> Result<JThrowable<'a>> {
        let class: JClass = DEFAULT_EXCEPTION_CLASS.lookup(jni)?;
        (class, self).lookup(jni)
    }
}

impl<'a, 'b> Desc<'a, JThrowable<'a>> for JNIString {
    //noinspection DuplicatedCode
    fn lookup(self, jni: &JNIEnv<'a>) -> Result<JThrowable<'a>> {
        let class: JClass = DEFAULT_EXCEPTION_CLASS.lookup(jni)?;
        (class, self).lookup(jni)
    }
}
