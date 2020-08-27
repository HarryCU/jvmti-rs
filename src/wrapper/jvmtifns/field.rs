use std::ptr;

use crate::{sys::*, objects::*, errors::*, JVMTIEnv, JSignature, JFieldName, to_bool};

impl<'a> JVMTIEnv<'a> {
    pub fn get_field_name(&self, klass: &JClass, field: &JFieldID) -> Result<JFieldName> {
        let mut name = ptr::null_mut();
        let mut signature = ptr::null_mut();
        let mut generic = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), GetFieldName,
            klass.into_inner(),
            field.into_inner(),
            &mut name,
            &mut signature,
            &mut generic
        );
        jvmti_error_code_to_result(res)?;

        let signature = JSignature::new(self.build_string(signature)?, self.build_string(generic)?)?;
        Ok(JFieldName::new(self.build_string(name)?, signature))
    }

    pub fn get_field_declaring_class(&self, klass: &JClass, field: &JFieldID) -> Result<JObject> {
        let mut value_ptr: jclass = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), GetFieldDeclaringClass,
            klass.into_inner(),
            field.into_inner(),
            &mut value_ptr
        );
        jvmti_error_code_to_result(res)?;
        Ok(value_ptr.into())
    }

    pub fn get_field_modifiers(&self, klass: &JClass, field: &JFieldID) -> Result<jint> {
        Ok(jvmti_call_number_result!(self.jvmti_raw(), jint,
            GetFieldModifiers,
            klass.into_inner(),
            field.into_inner()
        ))
    }

    pub fn is_field_synthetic(&self, klass: &JClass, field: &JFieldID) -> Result<bool> {
        let res = jvmti_call_number_result!(self.jvmti_raw(), jboolean,
            IsFieldSynthetic,
            klass.into_inner(),
            field.into_inner()
        );
        Ok(to_bool(res))
    }
}
