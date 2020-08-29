use crate::{sys::*, errors::*, objects::*, JvmtiError, JvmtiVerboseFlag, JvmtiJlocationFormat, JvmtiPhase, JVMTIFacadeEnv};

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn dispose_environment(&self) -> Result<()> {
        self.jvmti_rust().dispose_environment()
    }

    pub fn get_version_number(&self) -> Result<jint> {
        self.jvmti_rust().get_version_number()
    }

    pub fn get_error_name(&self, error: JvmtiError) -> Result<String> {
        self.jvmti_rust().get_error_name(error)
    }

    pub fn get_environment_local_storage(&self) -> Result<JLocalStorage> {
        self.jvmti_rust().get_environment_local_storage()
    }

    pub fn set_environment_local_storage(&self, data: &JLocalStorage) -> Result<()> {
        self.jvmti_rust().set_environment_local_storage(data)
    }

    pub fn set_verbose_flag(&self, flag: JvmtiVerboseFlag, value: bool) -> Result<()> {
        self.jvmti_rust().set_verbose_flag(flag, value)
    }

    pub fn get_jlocation_format(&self) -> Result<JvmtiJlocationFormat> {
        self.jvmti_rust().get_jlocation_format()
    }

    pub fn get_phase(&self) -> Result<JvmtiPhase> {
        self.jvmti_rust().get_phase()
    }
}
