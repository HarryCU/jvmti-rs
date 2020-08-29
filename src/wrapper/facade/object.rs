use crate::{sys::*, errors::*, objects::*, JVMTIFacadeEnv};

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn get_object_size(&self, obj: &JObject) -> Result<jlong> {
        self.jvmti_rust().get_object_size(obj)
    }

    pub fn get_object_hash_code(&self, obj: &JObject) -> Result<jint> {
        self.jvmti_rust().get_object_hash_code(obj)
    }

    pub fn get_object_monitor_usage(&self, obj: &JObject) -> Result<JMonitorUsage> {
        self.jvmti_rust().get_object_monitor_usage(obj)
    }
}
