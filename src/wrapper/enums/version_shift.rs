use crate::sys;

#[derive(Debug)]
pub enum JvmtiVersionShift {
    Major,
    Minor,
    Micro,
    Unsupported(sys::jvmtiVersionShift),
}


impl From<sys::jvmtiVersionShift> for JvmtiVersionShift {
    fn from(value: sys::jvmtiVersionShift) -> Self {
        match value {
            sys::JVMTI_VERSION_SHIFT_MAJOR => JvmtiVersionShift::Major,
            sys::JVMTI_VERSION_SHIFT_MINOR => JvmtiVersionShift::Minor,
            sys::JVMTI_VERSION_SHIFT_MICRO => JvmtiVersionShift::Micro,
            _ => JvmtiVersionShift::Unsupported(value)
        }
    }
}

impl From<JvmtiVersionShift> for sys::jvmtiVersionShift {
    fn from(value: JvmtiVersionShift) -> Self {
        match value {
            JvmtiVersionShift::Major => sys::JVMTI_VERSION_SHIFT_MAJOR,
            JvmtiVersionShift::Minor => sys::JVMTI_VERSION_SHIFT_MINOR,
            JvmtiVersionShift::Micro => sys::JVMTI_VERSION_SHIFT_MICRO,
            _ => sys::JVMTI_CONSTANT_UNSUPPORTED,
        }
    }
}