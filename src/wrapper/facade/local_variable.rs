use crate::{sys::*, errors::*, objects::*, JVMTIFacadeEnv};

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn get_local_instance(&self, thread: &JThreadID, depth: jint) -> Result<Option<JObject>> {
        self.jvmti_rust().get_local_instance(thread, depth)
    }

    pub fn get_local_object(&self, thread: &JThreadID, depth: jint, slot: jint) -> Result<Option<JObject>> {
        self.jvmti_rust().get_local_object(thread, depth, slot)
    }

    pub fn get_local_int(&self, thread: &JThreadID, depth: jint, slot: jint) -> Result<jint> {
        self.jvmti_rust().get_local_int(thread, depth, slot)
    }

    pub fn get_local_long(&self, thread: &JThreadID, depth: jint, slot: jint) -> Result<jlong> {
        self.jvmti_rust().get_local_long(thread, depth, slot)
    }

    pub fn get_local_float(&self, thread: &JThreadID, depth: jint, slot: jint) -> Result<jfloat> {
        self.jvmti_rust().get_local_float(thread, depth, slot)
    }

    pub fn get_local_double(&self, thread: &JThreadID, depth: jint, slot: jint) -> Result<jdouble> {
        self.jvmti_rust().get_local_double(thread, depth, slot)
    }

    pub fn set_local_object(&self, thread: &JThreadID, depth: jint, slot: jint, obj: &Option<JObject>) -> Result<()> {
        self.jvmti_rust().set_local_object(thread, depth, slot, obj)
    }

    pub fn set_local_int(&self, thread: &JThreadID, depth: jint, slot: jint, value: jint) -> Result<()> {
        self.jvmti_rust().set_local_int(thread, depth, slot, value)
    }

    pub fn set_local_long(&self, thread: &JThreadID, depth: jint, slot: jint, value: jlong) -> Result<()> {
        self.jvmti_rust().set_local_long(thread, depth, slot, value)
    }

    pub fn set_local_float(&self, thread: &JThreadID, depth: jint, slot: jint, value: jfloat) -> Result<()> {
        self.jvmti_rust().set_local_float(thread, depth, slot, value)
    }

    pub fn set_local_double(&self, thread: &JThreadID, depth: jint, slot: jint, value: jdouble) -> Result<()> {
        self.jvmti_rust().set_local_double(thread, depth, slot, value)
    }
}
