use crate::{sys::*, errors::*, objects::*, JVMTIFacadeEnv};

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn get_tag(&self, object: &JObject) -> Result<jlong> {
        self.jvmti_rust().get_tag(object)
    }

    pub fn set_tag(&self, object: &JObject, tag: jlong) -> Result<()> {
        self.jvmti_rust().set_tag(object, tag)
    }

    pub fn get_objects_with_tags(&self, tags: &Vec<jlong>) -> Result<Vec<JTagObject>> {
        self.jvmti_rust().get_objects_with_tags(tags)
    }

    pub fn force_garbage_collection(&self) -> Result<()> {
        self.jvmti_rust().force_garbage_collection()
    }
}
