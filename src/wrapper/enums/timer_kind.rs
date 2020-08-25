use crate::sys;

#[derive(Debug)]
pub enum JvmtiTimerKind {
    UserCpu,
    TotalCpu,
    Elapsed,
    Unsupported(sys::jvmtiTimerKind),
}


impl From<sys::jvmtiTimerKind> for JvmtiTimerKind {
    fn from(value: sys::jvmtiTimerKind) -> Self {
        match value {
            sys::JVMTI_TIMER_USER_CPU => JvmtiTimerKind::UserCpu,
            sys::JVMTI_TIMER_TOTAL_CPU => JvmtiTimerKind::TotalCpu,
            sys::JVMTI_TIMER_ELAPSED => JvmtiTimerKind::Elapsed,
            _ => JvmtiTimerKind::Unsupported(value)
        }
    }
}

impl From<JvmtiTimerKind> for sys::jvmtiTimerKind {
    fn from(value: JvmtiTimerKind) -> Self {
        match value {
            JvmtiTimerKind::UserCpu => sys::JVMTI_TIMER_USER_CPU,
            JvmtiTimerKind::TotalCpu => sys::JVMTI_TIMER_TOTAL_CPU,
            JvmtiTimerKind::Elapsed => sys::JVMTI_TIMER_ELAPSED,
            _ => sys::JVMTI_CONSTANT_UNSUPPORTED,
        }
    }
}