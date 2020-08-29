use jni::sys::jobject;
use jni::sys::jclass;
use crate::{builder::*, errors::*, JVMTIEnv, objects::*, sys::*};

impl<'a> JVMTIEnv<'a> {
    pub fn get_loaded_classes(&self) -> Result<Vec<JClass>> {
        let mut builder: MutAutoDeallocateObjectArrayBuilder<jclass> = MutAutoDeallocateObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.jvmti_raw(), GetLoadedClasses,
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build(self))
    }

    pub fn get_class_loader_classes(&self, initiating_loader: &JObject) -> Result<Vec<JClass>> {
        let mut builder: MutAutoDeallocateObjectArrayBuilder<jclass> = MutAutoDeallocateObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.jvmti_raw(), GetClassLoaderClasses,
            initiating_loader.into_inner(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build(self))
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
}
