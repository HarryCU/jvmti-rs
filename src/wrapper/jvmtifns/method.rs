use std::ptr;

use crate::{sys::*, errors::*, builder::*, objects::*, JVMTIEnv, JMethodName, JSignature, to_bool, AdapterTransform};
use crate::sys;
use jni::strings::JNIString;
use std::os::raw::c_char;

impl<'a> JVMTIEnv<'a> {
    pub fn set_native_method_prefix<S>(&self, prefix: S) -> Result<()>
        where
            S: Into<JNIString> {
        let value = prefix.into();
        jvmti_call!(self.jvmti_raw(), SetNativeMethodPrefix,
            value.as_ptr()
        )
    }

    pub fn set_native_method_prefixes<S>(&self, prefixes: &Vec<S>) -> Result<()>
        where
            S: Into<JNIString> + AsRef<str> {
        if prefixes.is_empty() {
            return Ok(());
        }

        let mut prefixes: Vec<*mut c_char> = prefixes.iter().map(|e| {
            let prefix: JNIString = e.into();
            prefix.as_ptr() as *mut c_char
        }).collect();

        jvmti_call!(self.jvmti_raw(), SetNativeMethodPrefixes,
            prefixes.len() as jint,
            prefixes.as_mut_ptr()
        )
    }

    pub fn get_method_name<M>(&self, method: M) -> Result<JMethodName>
        where
            M: AdapterTransform<jmethodID> {
        let mut name = ptr::null_mut();
        let mut signature = ptr::null_mut();
        let mut generic = ptr::null_mut();

        let res = jvmti_call_result!(self.jvmti_raw(), GetMethodName,
            method.transform(),
            &mut name,
            &mut signature,
            &mut generic
        );
        jvmti_error_code_to_result(res)?;

        let signature = JSignature::new(self.build_string(signature)?, self.build_string(generic)?)?;
        Ok(JMethodName::new(self.build_string(name)?, signature))
    }

    pub fn get_method_declaring_class<M>(&self, method: M) -> Result<JObject>
        where
            M: AdapterTransform<jmethodID> {
        let mut value_ptr: jclass = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), GetMethodDeclaringClass,
            method.transform(),
            &mut value_ptr
        );
        jvmti_error_code_to_result(res)?;
        Ok(value_ptr.into())
    }

    pub fn get_method_modifiers<M>(&self, method: M) -> Result<jint>
        where
            M: AdapterTransform<jmethodID> {
        Ok(jvmti_call_number_result!(self.jvmti_raw(), jint,
            GetMethodModifiers,
            method.transform()
        ))
    }

    pub fn get_max_locals<M>(&self, method: M) -> Result<jint>
        where
            M: AdapterTransform<jmethodID> {
        Ok(jvmti_call_number_result!(self.jvmti_raw(), jint,
            GetMaxLocals,
            method.transform()
        ))
    }

    pub fn get_arguments_size<M>(&self, method: M) -> Result<jint>
        where
            M: AdapterTransform<jmethodID> {
        Ok(jvmti_call_number_result!(self.jvmti_raw(), jint,
            GetArgumentsSize,
            method.transform()
        ))
    }

    pub fn get_line_number_table<M>(&self, method: M) -> Result<Vec<JLineNumberEntry>>
        where
            M: AdapterTransform<jmethodID> {
        let mut builder: MutObjectArrayBuilder<jvmtiLineNumberEntry> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.jvmti_raw(), GetLineNumberTable,
            method.transform(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build())
    }

    pub fn get_method_location<M>(&self, method: M) -> Result<JMethodLocation>
        where
            M: AdapterTransform<jmethodID> {
        let mut start_location: sys::jlocation = 0 as sys::jlocation;
        let mut end_location: sys::jlocation = 0 as sys::jlocation;
        let res = jvmti_call_result!(self.jvmti_raw(), GetMethodLocation,
            method.transform(),
            &mut start_location,
            &mut end_location
        );
        jvmti_error_code_to_result(res)?;
        Ok(JMethodLocation::new(start_location, end_location))
    }

    pub fn get_local_variable_table<M>(&self, method: M) -> Result<Vec<JLocalVariableEntry>>
        where
            M: AdapterTransform<jmethodID> {
        let mut builder: MutObjectArrayBuilder<jvmtiLocalVariableEntry> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.jvmti_raw(), GetLocalVariableTable,
            method.transform(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build(self))
    }

    pub fn get_bytecodes<M>(&self, method: M) -> Result<JMemoryAllocate>
        where
            M: AdapterTransform<jmethodID> {
        let mut bytecode_count: jint = 0 as jint;
        let mut bytecodes_ptr: jmemory = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), GetBytecodes,
            method.transform(),
            &mut bytecode_count,
            &mut bytecodes_ptr
        );
        jvmti_error_code_to_result(res)?;
        Ok(JMemoryAllocate::new(bytecodes_ptr, bytecode_count as jlong, self))
    }

    pub fn is_method_native<M>(&self, method: M) -> Result<bool>
        where
            M: AdapterTransform<jmethodID> {
        let res = jvmti_call_number_result!(self.jvmti_raw(), jboolean,
            IsMethodNative,
            method.transform()
        );
        Ok(to_bool(res))
    }

    pub fn is_method_synthetic<M>(&self, method: M) -> Result<bool>
        where
            M: AdapterTransform<jmethodID> {
        let res = jvmti_call_number_result!(self.jvmti_raw(), jboolean,
            IsMethodSynthetic,
            method.transform()
        );
        Ok(to_bool(res))
    }

    pub fn is_method_obsolete<M>(&self, method: M) -> Result<bool>
        where
            M: AdapterTransform<jmethodID> {
        let res = jvmti_call_number_result!(self.jvmti_raw(), jboolean,
            IsMethodObsolete,
            method.transform()
        );
        Ok(to_bool(res))
    }
}
