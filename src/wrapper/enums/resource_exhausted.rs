use crate::sys;

#[derive(Debug)]
pub enum JvmtiResourceExhausted {
    OomError,
    JavaHeap,
    Threads,
    Unsupported(sys::jvmtiResourceExhausted),
}


impl From<sys::jvmtiResourceExhausted> for JvmtiResourceExhausted {
    fn from(value: sys::jvmtiResourceExhausted) -> Self {
        match value {
            sys::JVMTI_RESOURCE_EXHAUSTED_OOM_ERROR => JvmtiResourceExhausted::OomError,
            sys::JVMTI_RESOURCE_EXHAUSTED_JAVA_HEAP => JvmtiResourceExhausted::JavaHeap,
            sys::JVMTI_RESOURCE_EXHAUSTED_THREADS => JvmtiResourceExhausted::Threads,
            _ => JvmtiResourceExhausted::Unsupported(value)
        }
    }
}

impl From<JvmtiResourceExhausted> for sys::jvmtiResourceExhausted {
    fn from(value: JvmtiResourceExhausted) -> Self {
        match value {
            JvmtiResourceExhausted::OomError => sys::JVMTI_RESOURCE_EXHAUSTED_OOM_ERROR,
            JvmtiResourceExhausted::JavaHeap => sys::JVMTI_RESOURCE_EXHAUSTED_JAVA_HEAP,
            JvmtiResourceExhausted::Threads => sys::JVMTI_RESOURCE_EXHAUSTED_THREADS,
            _ => sys::JVMTI_CONSTANT_UNSUPPORTED,
        }
    }
}