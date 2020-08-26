use std::ptr;

use jni_sys::{jmethodID, jfieldID, jclass};
use crate::wrapper::{
    errors::*,
    enums::*,
    objects::*,
    utils::*,
    builder::*,
    JVMTIEnv,
    JSignature,
};
use crate::sys::{jboolean, JClass, JObject, jobject, jvmtiClassDefinition, jvmtiClassStatus, jint, JMethodID, JFieldID, jmemory};

impl<'a> JVMTIEnv<'a> {
    pub fn get_loaded_classes(&self) -> Result<Vec<JClass>> {
        let mut builder: MutObjectArrayBuilder<jclass> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.jvmti_raw(), GetLoadedClasses,
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build())
    }

    pub fn get_class_loader_classes(&self, initiating_loader: &JObject) -> Result<Vec<JClass>> {
        let mut builder: MutObjectArrayBuilder<jclass> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.jvmti_raw(), GetClassLoaderClasses,
            initiating_loader.into_inner(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build())
    }

    pub fn retransform_classes(&self, classes: &Vec<JClass>) -> Result<()> {
        if classes.is_empty() {
            return Ok(());
        }
        let items: Vec<jobject> = classes.iter().map(|&e| e.into_inner()).collect();
        jvmti_call!( self.jvmti_raw(), RetransformClasses,
            items.len() as jint,
            items.as_ptr()
        )
    }

    pub fn redefine_classes(&self, classes: &Vec<JClassDefinition>) -> Result<()> {
        if classes.is_empty() {
            return Ok(());
        }
        let definitions: Vec<jvmtiClassDefinition> = classes.iter().map(|e| (e).into()).collect();
        jvmti_call!(self.jvmti_raw(), RedefineClasses,
            definitions.len() as jint,
            definitions.as_ptr()
        )
    }

    pub fn get_class_signature(&self, klass: &JClass) -> Result<JSignature> {
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

    pub fn get_class_status(&self, klass: &JClass) -> Result<JvmtiClassStatus> {
        let res = jvmti_call_number_result!(self.jvmti_raw(), jint,
            GetClassStatus,
            klass.into_inner()
        );
        Ok((res as jvmtiClassStatus).into())
    }

    pub fn get_source_file_name(&self, klass: &JClass) -> Result<JString> {
        let mut source_name = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), GetSourceFileName,
            klass.into_inner(),
            &mut source_name
        );
        jvmti_error_code_to_result(res)?;
        self.build_string(source_name)
    }

    pub fn get_class_modifiers(&self, klass: &JClass) -> Result<jint> {
        Ok(jvmti_call_number_result!(self.jvmti_raw(), jint,
            GetClassModifiers,
            klass.into_inner()
        ))
    }

    pub fn get_class_methods(&self, klass: &JClass) -> Result<Vec<JMethodID>> {
        let mut builder: MutObjectArrayBuilder<jmethodID> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.jvmti_raw(), GetClassMethods,
            klass.into_inner(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build())
    }

    pub fn get_class_fields(&self, klass: &JClass) -> Result<Vec<JFieldID>> {
        let mut builder: MutObjectArrayBuilder<jfieldID> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.jvmti_raw(), GetClassFields,
            klass.into_inner(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build())
    }

    pub fn get_implemented_interfaces(&self, klass: &JClass) -> Result<Vec<JClass>> {
        let mut builder: MutObjectArrayBuilder<jclass> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.jvmti_raw(), GetImplementedInterfaces,
            klass.into_inner(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build())
    }

    pub fn is_interface(&self, klass: &JClass) -> Result<bool> {
        let res = jvmti_call_number_result!(self.jvmti_raw(), jboolean,
            IsInterface,
            klass.into_inner()
        );
        Ok(to_bool(res))
    }

    pub fn is_array_class(&self, klass: &JClass) -> Result<bool> {
        let res = jvmti_call_number_result!(self.jvmti_raw(), jboolean,
            IsArrayClass,
            klass.into_inner()
        );
        Ok(to_bool(res))
    }

    pub fn is_modifiable_class(&self, klass: &JClass) -> Result<bool> {
        let res = jvmti_call_number_result!(self.jvmti_raw(), jboolean,
            IsModifiableClass,
            klass.into_inner()
        );
        Ok(to_bool(res))
    }

    pub fn get_source_debug_extension(&self, klass: &JClass) -> Result<JString> {
        let mut source_debug_extension = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), GetSourceDebugExtension,
            klass.into_inner(),
            &mut source_debug_extension
        );
        jvmti_error_code_to_result(res)?;
        self.build_string(source_debug_extension)
    }

    pub fn get_class_loader(&self, klass: &JClass) -> Result<JClassLoader> {
        let mut value_ptr: jobject = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), GetClassLoader,
            klass.into_inner(),
            &mut value_ptr
        );
        jvmti_error_code_to_result(res)?;
        return Ok(value_ptr.into());
    }

    pub fn get_constant_pool(&self, klass: &JClass) -> Result<JConstantPool> {
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

    pub fn get_class_version_numbers(&self, klass: &JClass) -> Result<JClassVersionNumber> {
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
