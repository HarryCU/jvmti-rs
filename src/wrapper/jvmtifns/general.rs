use std::{
    ffi::c_void,
    ptr,
};

use crate::{sys::*, errors::*, objects::*, JVMTIEnv, JvmtiError, JvmtiVerboseFlag, JvmtiJlocationFormat, JvmtiPhase, to_jboolean};

impl<'a> JVMTIEnv<'a> {
    pub fn dispose_environment(&self) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), DisposeEnvironment)
    }

    pub fn get_version_number(&self) -> Result<jint> {
        Ok(jvmti_call_number_result!(self.jvmti_raw(), jint,
            GetVersionNumber
        ))
    }

    pub fn get_error_name(&self, error: JvmtiError) -> Result<JvmtiString> {
        let mut name = ptr::null_mut();
        let err: jvmtiError = error.into();
        let res = jvmti_call_result!(self.jvmti_raw(), GetErrorName,
            err,
            &mut name
        );
        jvmti_error_code_to_result(res)?;
        self.build_string(name)
    }

    pub fn get_environment_local_storage(&self) -> Result<JLocalStorage> {
        let mut data_ptr: *mut c_void = ptr::null_mut() as *mut c_void;
        let res = jvmti_call_result!(self.jvmti_raw(), GetEnvironmentLocalStorage,
            &mut data_ptr
        );
        jvmti_error_code_to_result(res)?;
        Ok(JLocalStorage::new(data_ptr))
    }

    pub fn set_environment_local_storage(&self, data: &JLocalStorage) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), SetEnvironmentLocalStorage,
            data.as_ptr()
        )
    }

    pub fn set_verbose_flag(&self, flag: JvmtiVerboseFlag, value: bool) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), SetVerboseFlag,
            flag.into(),
            to_jboolean(value)
        )
    }

    pub fn get_jlocation_format(&self) -> Result<JvmtiJlocationFormat> {
        let res = jvmti_call_number_result!(self.jvmti_raw(), jvmtiJlocationFormat,
            GetJLocationFormat
        );
        Ok(res.into())
    }

    pub fn get_phase(&self) -> Result<JvmtiPhase> {
        let res = jvmti_call_number_result!(self.jvmti_raw(), jvmtiPhase,
            GetPhase
        );
        Ok(res.into())
    }
}
