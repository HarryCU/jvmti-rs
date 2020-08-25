use crate::sys;

#[derive(Debug)]
pub enum JvmtiVerboseFlag {
    Other,
    Gc,
    Class,
    Jni,
    Unsupported(sys::jvmtiVerboseFlag),
}


impl From<sys::jvmtiVerboseFlag> for JvmtiVerboseFlag {
    fn from(value: sys::jvmtiVerboseFlag) -> Self {
        match value {
            sys::JVMTI_VERBOSE_OTHER => JvmtiVerboseFlag::Other,
            sys::JVMTI_VERBOSE_GC => JvmtiVerboseFlag::Gc,
            sys::JVMTI_VERBOSE_CLASS => JvmtiVerboseFlag::Class,
            sys::JVMTI_VERBOSE_JNI => JvmtiVerboseFlag::Jni,
            _ => JvmtiVerboseFlag::Unsupported(value)
        }
    }
}

impl From<JvmtiVerboseFlag> for sys::jvmtiVerboseFlag {
    fn from(value: JvmtiVerboseFlag) -> Self {
        match value {
            JvmtiVerboseFlag::Other => sys::JVMTI_VERBOSE_OTHER,
            JvmtiVerboseFlag::Gc => sys::JVMTI_VERBOSE_GC,
            JvmtiVerboseFlag::Class => sys::JVMTI_VERBOSE_CLASS,
            JvmtiVerboseFlag::Jni => sys::JVMTI_VERBOSE_JNI,
            _ => sys::JVMTI_CONSTANT_UNSUPPORTED,
        }
    }
}