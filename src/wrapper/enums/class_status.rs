use crate::sys;

#[derive(Debug)]
pub enum JvmtiClassStatus {
    Verified,
    Prepared,
    Initialized,
    Error,
    Array,
    Primitive,
    Unsupported(sys::jvmtiClassStatus),
}

impl From<sys::jvmtiClassStatus> for JvmtiClassStatus {
    fn from(value: sys::jvmtiClassStatus) -> Self {
        match value {
            sys::JVMTI_CLASS_STATUS_VERIFIED => JvmtiClassStatus::Verified,
            sys::JVMTI_CLASS_STATUS_PREPARED => JvmtiClassStatus::Prepared,
            sys::JVMTI_CLASS_STATUS_INITIALIZED => JvmtiClassStatus::Initialized,
            sys::JVMTI_CLASS_STATUS_ERROR => JvmtiClassStatus::Error,
            sys::JVMTI_CLASS_STATUS_ARRAY => JvmtiClassStatus::Array,
            sys::JVMTI_CLASS_STATUS_PRIMITIVE => JvmtiClassStatus::Primitive,
            _ => JvmtiClassStatus::Unsupported(value)
        }
    }
}

impl From<JvmtiClassStatus> for sys::jvmtiClassStatus {
    fn from(value: JvmtiClassStatus) -> Self {
        match value {
            JvmtiClassStatus::Verified => sys::JVMTI_CLASS_STATUS_VERIFIED,
            JvmtiClassStatus::Prepared => sys::JVMTI_CLASS_STATUS_PREPARED,
            JvmtiClassStatus::Initialized => sys::JVMTI_CLASS_STATUS_INITIALIZED,
            JvmtiClassStatus::Error => sys::JVMTI_CLASS_STATUS_ERROR,
            JvmtiClassStatus::Array => sys::JVMTI_CLASS_STATUS_ARRAY,
            JvmtiClassStatus::Primitive => sys::JVMTI_CLASS_STATUS_PRIMITIVE,
            _ => sys::JVMTI_CONSTANT_UNSUPPORTED,
        }
    }
}