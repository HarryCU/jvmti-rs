use std::ptr;

use crate::{sys::*, objects::*, errors::*, JVMTIEnv, JSignature, JFieldName, to_bool, Desc, Transform};
use jni::strings::JNIString;

impl<'a> JVMTIEnv<'a> {
    pub fn get_field_id<K, F, V>(&self, class: K, name: F, sig: V) -> Result<(JClass, JFieldID)>
        where
            K: Into<JNIString>,
            F: Into<JNIString>,
            V: Into<JNIString> {
        let class_name = class.into();

        let klass: JClass = class_name.lookup(self)?;
        Ok((klass, (klass, name, sig).lookup(self)?))
    }

    pub fn get_static_field_id<K, F, V>(&self, class: K, name: F, sig: V) -> Result<(JClass, JStaticFieldID)>
        where
            K: Into<JNIString>,
            F: Into<JNIString>,
            V: Into<JNIString> {
        let class_name = class.into();

        let klass: JClass = class_name.lookup(self)?;
        Ok((klass, (klass, name, sig).lookup(self)?))
    }

    pub fn get_field_name<K, F>(&self, class: K, field: F) -> Result<JFieldName>
        where
            K: Transform<'a, JClass<'a>>,
            F: Transform<'a, jfieldID> {
        let klass: JClass = class.transform(self)?;

        let mut name = ptr::null_mut();
        let mut signature = ptr::null_mut();
        let mut generic = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), GetFieldName,
            klass.into_inner(),
            field.transform(self)?,
            &mut name,
            &mut signature,
            &mut generic
        );
        jvmti_error_code_to_result(res)?;

        let signature = JSignature::new(self.build_string(signature)?, self.build_string(generic)?)?;
        Ok(JFieldName::new(self.build_string(name)?, signature))
    }

    pub fn get_field_declaring_class<K, F>(&self, class: K, field: F) -> Result<JObject>
        where
            K: Transform<'a, JClass<'a>>,
            F: Transform<'a, jfieldID> {
        let klass: JClass = class.transform(self)?;

        let mut value_ptr: jclass = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), GetFieldDeclaringClass,
            klass.into_inner(),
            field.transform(self)?,
            &mut value_ptr
        );
        jvmti_error_code_to_result(res)?;
        Ok(value_ptr.into())
    }

    pub fn get_field_modifiers<K, F>(&self, class: K, field: F) -> Result<jint>
        where
            K: Transform<'a, JClass<'a>>,
            F: Transform<'a, jfieldID> {
        let klass: JClass = class.transform(self)?;

        Ok(jvmti_call_number_result!(self.jvmti_raw(), jint,
            GetFieldModifiers,
            klass.into_inner(),
            field.transform(self)?
        ))
    }

    pub fn is_field_synthetic<K, F>(&self, class: K, field: F) -> Result<bool>
        where
            K: Transform<'a, JClass<'a>>,
            F: Transform<'a, jfieldID> {
        let klass: JClass = class.transform(self)?;

        let res = jvmti_call_number_result!(self.jvmti_raw(), jboolean,
            IsFieldSynthetic,
            klass.into_inner(),
            field.transform(self)?
        );
        Ok(to_bool(res))
    }
}
