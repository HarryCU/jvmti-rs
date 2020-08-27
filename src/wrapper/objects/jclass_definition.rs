use crate::{sys::*, objects::*, JVMTIEnv};

#[derive(Clone)]
pub struct JClassDefinition<'a> {
    pub klass: JClass<'a>,
    pub class_bytes: JMemoryAllocate<'a>,
}

impl<'a> JClassDefinition<'a> {
    pub fn new<'b: 'a>(klass: JClass<'a>, code_bytes: jmemory,
                       size: jlong,
                       env: &'b JVMTIEnv<'a>) -> JClassDefinition<'a> {
        let class_bytes = JMemoryAllocate::new(code_bytes, size, env);
        JClassDefinition {
            klass,
            class_bytes,
        }
    }
}

impl<'a> From<&JClassDefinition<'a>> for jvmtiClassDefinition {
    fn from(definition: &JClassDefinition<'a>) -> Self {
        jvmtiClassDefinition {
            klass: definition.klass.into_inner(),
            class_byte_count: definition.class_bytes.size as jint,
            class_bytes: definition.class_bytes.ptr,
        }
    }
}

impl<'a> Drop for JClassDefinition<'a> {
    fn drop(&mut self) {
        let bytes = &mut self.class_bytes;
        bytes.deallocate()
    }
}