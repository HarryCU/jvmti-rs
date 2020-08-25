use crate::sys;

#[derive(Debug, Clone, Copy)]
pub enum JvmtiEventMode {
    Enable,
    Disable,
    Unsupported(sys::jvmtiEventMode),
}


impl From<sys::jvmtiEventMode> for JvmtiEventMode {
    fn from(value: sys::jvmtiEventMode) -> Self {
        match value {
            sys::JVMTI_ENABLE => JvmtiEventMode::Enable,
            sys::JVMTI_DISABLE => JvmtiEventMode::Disable,
            _ => JvmtiEventMode::Unsupported(value)
        }
    }
}

impl From<JvmtiEventMode> for sys::jvmtiEventMode {
    fn from(value: JvmtiEventMode) -> Self {
        match value {
            JvmtiEventMode::Enable => sys::JVMTI_ENABLE,
            JvmtiEventMode::Disable => sys::JVMTI_DISABLE,
            _ => sys::JVMTI_CONSTANT_UNSUPPORTED,
        }
    }
}