use crate::wrapper::{
    errors::*,
    JVMTIEnv,
};
use crate::sys::{JClass, JFieldID};

impl<'a> JVMTIEnv<'a> {
    pub fn set_field_access_watch(&self, klass: &JClass, field: &JFieldID) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), SetFieldAccessWatch,
            klass.into_inner(),
            field.into_inner()
        )
    }

    pub fn clear_field_access_watch(&self, klass: &JClass, field: &JFieldID) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), ClearFieldAccessWatch,
            klass.into_inner(),
            field.into_inner()
        )
    }

    pub fn set_field_modification_watch(&self, klass: &JClass, field: &JFieldID) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), SetFieldModificationWatch,
            klass.into_inner(),
            field.into_inner()
        )
    }

    pub fn clear_field_modification_watch(&self, klass: &JClass, field: &JFieldID) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), ClearFieldModificationWatch,
            klass.into_inner(),
            field.into_inner()
        )
    }
}
