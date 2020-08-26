use std::ptr;

use crate::wrapper::{errors::*, objects::*, utils::*, builder::*, JVMTIEnv, JMethodName, JSignature};
use crate::sys;
use crate::sys::{jboolean, jclass, JMethodID, JObject, jint, jvmtiLocalVariableEntry, JNIString, jvmtiLineNumberEntry, jmemory, jlong};


impl<'a> JVMTIEnv<'a> {
    pub fn get_method_name(&self, method: &JMethodID) -> Result<JMethodName> {
        let mut name = ptr::null_mut();
        let mut signature = ptr::null_mut();
        let mut generic = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), GetMethodName,
            method.into_inner(),
            &mut name,
            &mut signature,
            &mut generic
        );
        jvmti_error_code_to_result(res)?;

        let signature = JSignature::new(self.build_string(signature)?, self.build_string(generic)?)?;
        Ok(JMethodName::new(self.build_string(name)?, signature))
    }

    pub fn get_method_declaring_class(&self, method: &JMethodID) -> Result<JObject> {
        let mut value_ptr: jclass = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), GetMethodDeclaringClass,
            method.into_inner(),
            &mut value_ptr
        );
        jvmti_error_code_to_result(res)?;
        Ok(value_ptr.into())
    }

    pub fn get_method_modifiers(&self, method: &JMethodID) -> Result<jint> {
        Ok(jvmti_call_number_result!(self.jvmti_raw(), jint,
            GetMethodModifiers,
            method.into_inner()
        ))
    }

    pub fn get_max_locals(&self, method: &JMethodID) -> Result<jint> {
        Ok(jvmti_call_number_result!(self.jvmti_raw(), jint,
            GetMaxLocals,
            method.into_inner()
        ))
    }

    pub fn get_arguments_size(&self, method: &JMethodID) -> Result<jint> {
        Ok(jvmti_call_number_result!(self.jvmti_raw(), jint,
            GetArgumentsSize,
            method.into_inner()
        ))
    }

    pub fn get_line_number_table(&self, method: &JMethodID) -> Result<Vec<JLineNumberEntry>> {
        let mut builder: MutObjectArrayBuilder<jvmtiLineNumberEntry> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.jvmti_raw(), GetLineNumberTable,
            method.into_inner(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build())
    }

    pub fn get_method_location(&self, method: &JMethodID) -> Result<JMethodLocation> {
        let mut start_location: sys::jlocation = 0 as sys::jlocation;
        let mut end_location: sys::jlocation = 0 as sys::jlocation;
        let res = jvmti_call_result!(self.jvmti_raw(), GetMethodLocation,
            method.into_inner(),
            &mut start_location,
            &mut end_location
        );
        jvmti_error_code_to_result(res)?;
        Ok(JMethodLocation::new(start_location, end_location))
    }

    pub fn get_local_variable_table(&self, method: &JMethodID) -> Result<Vec<JLocalVariableEntry>> {
        let mut builder: MutObjectArrayBuilder<jvmtiLocalVariableEntry> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.jvmti_raw(), GetLocalVariableTable,
            method.into_inner(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build(self))
    }

    pub fn set_native_method_prefix<S>(&self, prefix: S) -> Result<()>
        where
            S: Into<JNIString> {
        let value = prefix.into();
        jvmti_call!(self.jvmti_raw(), SetNativeMethodPrefix,
            value.as_ptr()
        )
    }

    pub fn get_bytecodes(&self, method: &JMethodID) -> Result<JMemoryAllocate> {
        let mut bytecode_count: jint = 0 as jint;
        let mut bytecodes_ptr: jmemory = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), GetBytecodes,
            method.into_inner(),
            &mut bytecode_count,
            &mut bytecodes_ptr
        );
        jvmti_error_code_to_result(res)?;
        Ok(JMemoryAllocate::new(bytecodes_ptr, bytecode_count as jlong, self))
    }

    pub fn is_method_native(&self, method: &JMethodID) -> Result<bool> {
        let res = jvmti_call_number_result!(self.jvmti_raw(), jboolean,
            IsMethodNative,
            method.into_inner()
        );
        Ok(to_bool(res))
    }

    pub fn is_method_synthetic(&self, method: &JMethodID) -> Result<bool> {
        let res = jvmti_call_number_result!(self.jvmti_raw(), jboolean,
            IsMethodSynthetic,
            method.into_inner()
        );
        Ok(to_bool(res))
    }

    pub fn is_method_obsolete(&self, method: &JMethodID) -> Result<bool> {
        let res = jvmti_call_number_result!(self.jvmti_raw(), jboolean,
            IsMethodObsolete,
            method.into_inner()
        );
        Ok(to_bool(res))
    }
}
