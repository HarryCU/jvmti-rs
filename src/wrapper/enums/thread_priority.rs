use crate::sys;

#[derive(Debug)]
pub enum JvmtiThreadPriority {
    Min,
    Norm,
    Max,
    Unsupported(sys::jvmtiThreadPriority),
}

impl From<sys::jvmtiThreadPriority> for JvmtiThreadPriority {
    fn from(value: sys::jvmtiThreadPriority) -> Self {
        match value {
            sys::JVMTI_THREAD_MIN_PRIORITY => JvmtiThreadPriority::Min,
            sys::JVMTI_THREAD_NORM_PRIORITY => JvmtiThreadPriority::Norm,
            sys::JVMTI_THREAD_MAX_PRIORITY => JvmtiThreadPriority::Max,
            _ => JvmtiThreadPriority::Unsupported(value)
        }
    }
}

impl From<JvmtiThreadPriority> for sys::jvmtiThreadPriority {
    fn from(value: JvmtiThreadPriority) -> Self {
        match value {
            JvmtiThreadPriority::Min => sys::JVMTI_THREAD_MIN_PRIORITY,
            JvmtiThreadPriority::Norm => sys::JVMTI_THREAD_NORM_PRIORITY,
            JvmtiThreadPriority::Max => sys::JVMTI_THREAD_MAX_PRIORITY,
            _ => sys::JVMTI_CONSTANT_UNSUPPORTED as i32,
        }
    }
}