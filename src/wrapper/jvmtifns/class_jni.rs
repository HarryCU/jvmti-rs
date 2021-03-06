use std::ptr;

use jni::strings::JNIString;
use jni::sys::jobject;
use jni::sys::jclass;
use crate::{builder::*, Desc, errors::*, JSignature, JvmtiClassStatus, JVMTIEnv, objects::*, sys::*, to_bool, Transform};

impl<'a> JVMTIEnv<'a> {
    pub fn get_class<S>(&self, jni: &jni::JNIEnv<'a>, name: S) -> Result<JClass>
        where
            S: Into<JNIString>, {
        let ffi_name = name.into();
        ffi_name.lookup(jni)
    }

    pub fn get_class_signature<K>(&self, jni: &jni::JNIEnv<'a>, class: K) -> Result<JSignature>
        where
            K: Transform<'a, JClass<'a>> {
        let klass: JClass = class.transform(jni)?;

        let mut signature = ptr::null_mut();
        let mut generic = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), GetClassSignature,
            klass.into_inner(),
            &mut signature,
            &mut generic
        );
        jvmti_error_code_to_result(res)?;

        JSignature::new(self.build_string(signature)?, self.build_string(generic)?)
    }

    pub fn get_class_status<K>(&self, jni: &jni::JNIEnv<'a>, class: K) -> Result<JvmtiClassStatus>
        where
            K: Transform<'a, JClass<'a>> {
        let klass: JClass = class.transform(jni)?;

        let res = jvmti_call_number_result!(self.jvmti_raw(), jint,
            GetClassStatus,
            klass.into_inner()
        );
        Ok((res as jvmtiClassStatus).into())
    }

    pub fn get_source_file_name<K>(&self, jni: &jni::JNIEnv<'a>, class: K) -> Result<String>
        where
            K: Transform<'a, JClass<'a>> {
        let klass: JClass = class.transform(jni)?;

        let mut source_name = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), GetSourceFileName,
            klass.into_inner(),
            &mut source_name
        );
        jvmti_error_code_to_result(res)?;
        Ok(self.build_string(source_name)?.into())
    }

    pub fn get_class_modifiers<K>(&self, jni: &jni::JNIEnv<'a>, class: K) -> Result<jint>
        where
            K: Transform<'a, JClass<'a>> {
        let klass: JClass = class.transform(jni)?;

        Ok(jvmti_call_number_result!(self.jvmti_raw(), jint,
            GetClassModifiers,
            klass.into_inner()
        ))
    }

    pub fn get_class_methods<K>(&self, jni: &jni::JNIEnv<'a>, class: K) -> Result<Vec<JMethodID>>
        where
            K: Transform<'a, JClass<'a>> {
        let klass: JClass = class.transform(jni)?;

        let mut builder: MutAutoDeallocateObjectArrayBuilder<jmethodID> = MutAutoDeallocateObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.jvmti_raw(), GetClassMethods,
            klass.into_inner(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build(self))
    }

    pub fn get_class_fields<K>(&self, jni: &jni::JNIEnv<'a>, class: K) -> Result<Vec<JFieldID>>
        where
            K: Transform<'a, JClass<'a>> {
        let klass: JClass = class.transform(jni)?;

        let mut builder: MutAutoDeallocateObjectArrayBuilder<jfieldID> = MutAutoDeallocateObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.jvmti_raw(), GetClassFields,
            klass.into_inner(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build(self))
    }

    pub fn get_implemented_interfaces<K>(&self, jni: &jni::JNIEnv<'a>, class: K) -> Result<Vec<JClass>>
        where
            K: Transform<'a, JClass<'a>> {
        let klass: JClass = class.transform(jni)?;

        let mut builder: MutAutoDeallocateObjectArrayBuilder<jclass> = MutAutoDeallocateObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.jvmti_raw(), GetImplementedInterfaces,
            klass.into_inner(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build(self))
    }

    pub fn is_interface<K>(&self, jni: &jni::JNIEnv<'a>, class: K) -> Result<bool>
        where
            K: Transform<'a, JClass<'a>> {
        let klass: JClass = class.transform(jni)?;

        let res = jvmti_call_number_result!(self.jvmti_raw(), jboolean,
            IsInterface,
            klass.into_inner()
        );
        Ok(to_bool(res))
    }

    pub fn is_array_class<K>(&self, jni: &jni::JNIEnv<'a>, class: K) -> Result<bool>
        where
            K: Transform<'a, JClass<'a>> {
        let klass: JClass = class.transform(jni)?;

        let res = jvmti_call_number_result!(self.jvmti_raw(), jboolean,
            IsArrayClass,
            klass.into_inner()
        );
        Ok(to_bool(res))
    }

    pub fn is_modifiable_class<K>(&self, jni: &jni::JNIEnv<'a>, class: K) -> Result<bool>
        where
            K: Transform<'a, JClass<'a>> {
        let klass: JClass = class.transform(jni)?;

        let res = jvmti_call_number_result!(self.jvmti_raw(), jboolean,
            IsModifiableClass,
            klass.into_inner()
        );
        Ok(to_bool(res))
    }

    pub fn get_source_debug_extension<K>(&self, jni: &jni::JNIEnv<'a>, class: K) -> Result<String>
        where
            K: Transform<'a, JClass<'a>> {
        let klass: JClass = class.transform(jni)?;

        let mut source_debug_extension = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), GetSourceDebugExtension,
            klass.into_inner(),
            &mut source_debug_extension
        );
        jvmti_error_code_to_result(res)?;
        Ok(self.build_string(source_debug_extension)?.into())
    }

    pub fn get_class_loader<K>(&self, jni: &jni::JNIEnv<'a>, class: K) -> Result<JClassLoader>
        where
            K: Transform<'a, JClass<'a>> {
        let klass: JClass = class.transform(jni)?;

        let mut value: jobject = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), GetClassLoader,
            klass.into_inner(),
            &mut value
        );
        jvmti_error_code_to_result(res)?;
        return Ok(value.into());
    }

    pub fn get_constant_pool<K>(&self, jni: &jni::JNIEnv<'a>, class: K) -> Result<JConstantPool>
        where
            K: Transform<'a, JClass<'a>> {
        let klass: JClass = class.transform(jni)?;

        let mut constant_pool_count: jint = 0 as jint;
        let mut constant_pool_byte_count: jint = 0 as jint;
        let mut constant_pool_bytes: jmemory = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), GetConstantPool,
            klass.into_inner(),
            &mut constant_pool_count,
            &mut constant_pool_byte_count,
            &mut constant_pool_bytes
        );
        jvmti_error_code_to_result(res)?;

        Ok(JConstantPool::new(constant_pool_count, constant_pool_byte_count, constant_pool_bytes, self))
    }

    pub fn get_class_version_numbers<K>(&self, jni: &jni::JNIEnv<'a>, class: K) -> Result<JClassVersionNumber>
        where
            K: Transform<'a, JClass<'a>> {
        let klass: JClass = class.transform(jni)?;

        let mut minor_version: jint = 0 as jint;
        let mut major_version: jint = 0 as jint;

        let res = jvmti_call_result!(self.jvmti_raw(), GetClassVersionNumbers,
            klass.into_inner(),
            &mut minor_version,
            &mut major_version
        );
        jvmti_error_code_to_result(res)?;
        Ok(JClassVersionNumber::new(minor_version, major_version))
    }
}
