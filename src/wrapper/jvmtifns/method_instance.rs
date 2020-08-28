use std::ptr;

use crate::{sys::*, errors::*, builder::*, objects::*, JVMTIEnv, to_bool, Transform};
use crate::sys;
use jni::strings::JNIString;

impl<'a> JVMTIEnv<'a> {
    pub fn get_method_declaring_class_i<K, M, V>(&self, jni: &jni::JNIEnv<'a>, class: K, name: M, sig: V) -> Result<JObject>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        let method = self.get_method_id(jni, class, name, sig)?;

        let mut value: jclass = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), GetMethodDeclaringClass,
            method.into_inner(),
            &mut value
        );
        jvmti_error_code_to_result(res)?;
        Ok(value.into())
    }

    pub fn get_method_modifiers_i<K, M, V>(&self, jni: &jni::JNIEnv<'a>, class: K, name: M, sig: V) -> Result<jint>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        let method = self.get_method_id(jni, class, name, sig)?;

        Ok(jvmti_call_number_result!(self.jvmti_raw(), jint,
            GetMethodModifiers,
            method.into_inner()
        ))
    }

    pub fn get_max_locals_i<K, M, V>(&self, jni: &jni::JNIEnv<'a>, class: K, name: M, sig: V) -> Result<jint>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        let method = self.get_method_id(jni, class, name, sig)?;

        Ok(jvmti_call_number_result!(self.jvmti_raw(), jint,
            GetMaxLocals,
            method.into_inner()
        ))
    }

    pub fn get_arguments_size_i<K, M, V>(&self, jni: &jni::JNIEnv<'a>, class: K, name: M, sig: V) -> Result<jint>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        let method = self.get_method_id(jni, class, name, sig)?;

        Ok(jvmti_call_number_result!(self.jvmti_raw(), jint,
            GetArgumentsSize,
            method.into_inner()
        ))
    }

    pub fn get_line_number_table_i<K, M, V>(&self, jni: &jni::JNIEnv<'a>, class: K, name: M, sig: V) -> Result<Vec<JLineNumberEntry>>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        let method = self.get_method_id(jni, class, name, sig)?;

        let mut builder: MutObjectArrayBuilder<jvmtiLineNumberEntry> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.jvmti_raw(), GetLineNumberTable,
            method.into_inner(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build())
    }

    pub fn get_method_location_i<K, M, V>(&self, jni: &jni::JNIEnv<'a>, class: K, name: M, sig: V) -> Result<JMethodLocation>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        let method = self.get_method_id(jni, class, name, sig)?;

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

    pub fn get_local_variable_table_i<K, M, V>(&self, jni: &jni::JNIEnv<'a>, class: K, name: M, sig: V) -> Result<Vec<JLocalVariableEntry>>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        let method = self.get_method_id(jni, class, name, sig)?;

        let mut builder: MutObjectArrayBuilder<jvmtiLocalVariableEntry> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.jvmti_raw(), GetLocalVariableTable,
            method.into_inner(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build(self))
    }

    pub fn get_bytecodes_i<K, M, V>(&self, jni: &jni::JNIEnv<'a>, class: K, name: M, sig: V) -> Result<JMemoryAllocate>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        let method = self.get_method_id(jni, class, name, sig)?;

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

    pub fn is_method_native_i<K, M, V>(&self, jni: &jni::JNIEnv<'a>, class: K, name: M, sig: V) -> Result<bool>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        let method = self.get_method_id(jni, class, name, sig)?;

        let res = jvmti_call_number_result!(self.jvmti_raw(), jboolean,
            IsMethodNative,
            method.into_inner()
        );
        Ok(to_bool(res))
    }

    pub fn is_method_synthetic_i<K, M, V>(&self, jni: &jni::JNIEnv<'a>, class: K, name: M, sig: V) -> Result<bool>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        let method = self.get_method_id(jni, class, name, sig)?;

        let res = jvmti_call_number_result!(self.jvmti_raw(), jboolean,
            IsMethodSynthetic,
            method.into_inner()
        );
        Ok(to_bool(res))
    }

    pub fn is_method_obsolete_i<K, M, V>(&self, jni: &jni::JNIEnv<'a>, class: K, name: M, sig: V) -> Result<bool>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        let method = self.get_method_id(jni, class, name, sig)?;

        let res = jvmti_call_number_result!(self.jvmti_raw(), jboolean,
            IsMethodObsolete,
            method.into_inner()
        );
        Ok(to_bool(res))
    }
}
