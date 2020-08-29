use crate::{errors::*, objects::*, JVMTIFacadeEnv};

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn get_loaded_classes(&self) -> Result<Vec<JClass>> {
        self.jvmti_rust().get_loaded_classes()
    }

    pub fn get_class_loader_classes(&self, initiating_loader: &JObject) -> Result<Vec<JClass>> {
        self.jvmti_rust().get_class_loader_classes(initiating_loader)
    }

    pub fn retransform_classes(&self, classes: &Vec<JClass>) -> Result<()> {
        self.jvmti_rust().retransform_classes(classes)
    }

    pub fn redefine_classes(&self, classes: &Vec<JClassDefinition>) -> Result<()> {
        self.jvmti_rust().redefine_classes(classes)
    }
}
