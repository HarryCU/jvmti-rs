use crate::sys;

#[derive(Debug)]
pub enum JvmtiVisit {
    Objects,
    Abort,
    Unsupported(sys::jvmtiVisit),
}


impl From<sys::jvmtiVisit> for JvmtiVisit {
    fn from(value: sys::jvmtiVisit) -> Self {
        match value {
            sys::JVMTI_VISIT_OBJECTS => JvmtiVisit::Objects,
            sys::JVMTI_VISIT_ABORT => JvmtiVisit::Abort,
            _ => JvmtiVisit::Unsupported(value)
        }
    }
}

impl From<JvmtiVisit> for sys::jvmtiVisit {
    fn from(value: JvmtiVisit) -> Self {
        match value {
            JvmtiVisit::Objects => sys::JVMTI_VISIT_OBJECTS,
            JvmtiVisit::Abort => sys::JVMTI_VISIT_ABORT,
            _ => sys::JVMTI_CONSTANT_UNSUPPORTED,
        }
    }
}