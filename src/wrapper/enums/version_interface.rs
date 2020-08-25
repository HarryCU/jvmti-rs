use crate::sys;

#[derive(Debug)]
pub enum JvmtiVersionInterface {
    Jni,
    Jvmti,
    Unsupported(sys::jvmtiVersionInterface),
}


impl From<sys::jvmtiVersionInterface> for JvmtiVersionInterface {
    fn from(value: sys::jvmtiVersionInterface) -> Self {
        match value {
            sys::JVMTI_VERSION_INTERFACE_JNI => JvmtiVersionInterface::Jni,
            sys::JVMTI_VERSION_INTERFACE_JVMTI => JvmtiVersionInterface::Jvmti,
            _ => JvmtiVersionInterface::Unsupported(value)
        }
    }
}

impl From<JvmtiVersionInterface> for sys::jvmtiVersionInterface {
    fn from(value: JvmtiVersionInterface) -> Self {
        match value {
            JvmtiVersionInterface::Jni => sys::JVMTI_VERSION_INTERFACE_JNI,
            JvmtiVersionInterface::Jvmti => sys::JVMTI_VERSION_INTERFACE_JVMTI,
            _ => sys::JVMTI_CONSTANT_UNSUPPORTED,
        }
    }
}