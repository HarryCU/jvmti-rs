use crate::sys;

#[derive(Debug)]
pub enum JvmtiVersionMask {
    InterfaceType,
    Major,
    Minor,
    Micro,
    Unsupported(sys::jvmtiVersionMask),
}


impl From<sys::jvmtiVersionMask> for JvmtiVersionMask {
    fn from(value: sys::jvmtiVersionMask) -> Self {
        match value {
            sys::JVMTI_VERSION_MASK_INTERFACE_TYPE => JvmtiVersionMask::InterfaceType,
            sys::JVMTI_VERSION_MASK_MAJOR => JvmtiVersionMask::Major,
            sys::JVMTI_VERSION_MASK_MINOR => JvmtiVersionMask::Minor,
            sys::JVMTI_VERSION_MASK_MICRO => JvmtiVersionMask::Micro,
            _ => JvmtiVersionMask::Unsupported(value)
        }
    }
}

impl From<JvmtiVersionMask> for sys::jvmtiVersionMask {
    fn from(value: JvmtiVersionMask) -> Self {
        match value {
            JvmtiVersionMask::InterfaceType => sys::JVMTI_VERSION_MASK_INTERFACE_TYPE,
            JvmtiVersionMask::Major => sys::JVMTI_VERSION_MASK_MAJOR,
            JvmtiVersionMask::Minor => sys::JVMTI_VERSION_MASK_MINOR,
            JvmtiVersionMask::Micro => sys::JVMTI_VERSION_MASK_MICRO,
            _ => sys::JVMTI_CONSTANT_UNSUPPORTED,
        }
    }
}