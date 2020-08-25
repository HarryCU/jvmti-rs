use crate::sys;

#[derive(Debug)]
pub enum JvmtiPhase {
    Onload,
    Primordial,
    Start,
    Live,
    Dead,
    Unsupported(sys::jvmtiPhase),
}

impl From<sys::jvmtiPhase> for JvmtiPhase {
    fn from(value: sys::jvmtiPhase) -> Self {
        match value {
            sys::JVMTI_PHASE_ONLOAD => JvmtiPhase::Onload,
            sys::JVMTI_PHASE_PRIMORDIAL => JvmtiPhase::Primordial,
            sys::JVMTI_PHASE_START => JvmtiPhase::Start,
            sys::JVMTI_PHASE_LIVE => JvmtiPhase::Live,
            sys::JVMTI_PHASE_DEAD => JvmtiPhase::Dead,
            _ => JvmtiPhase::Unsupported(value)
        }
    }
}

impl From<JvmtiPhase> for sys::jvmtiPhase {
    fn from(value: JvmtiPhase) -> Self {
        match value {
            JvmtiPhase::Onload => sys::JVMTI_PHASE_ONLOAD,
            JvmtiPhase::Primordial => sys::JVMTI_PHASE_PRIMORDIAL,
            JvmtiPhase::Start => sys::JVMTI_PHASE_START,
            JvmtiPhase::Live => sys::JVMTI_PHASE_LIVE,
            JvmtiPhase::Dead => sys::JVMTI_PHASE_DEAD,
            _ => sys::JVMTI_CONSTANT_UNSUPPORTED,
        }
    }
}