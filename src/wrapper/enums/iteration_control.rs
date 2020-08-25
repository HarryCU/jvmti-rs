use crate::sys;

#[derive(Debug)]
pub enum JvmtiIterationControl {
    Continue,
    Ignore,
    Abort,
    Unsupported(sys::jvmtiIterationControl),
}


impl From<sys::jvmtiIterationControl> for JvmtiIterationControl {
    fn from(value: sys::jvmtiIterationControl) -> Self {
        match value {
            sys::JVMTI_ITERATION_CONTINUE => JvmtiIterationControl::Continue,
            sys::JVMTI_ITERATION_IGNORE => JvmtiIterationControl::Ignore,
            sys::JVMTI_ITERATION_ABORT => JvmtiIterationControl::Abort,
            _ => JvmtiIterationControl::Unsupported(value)
        }
    }
}

impl From<JvmtiIterationControl> for sys::jvmtiIterationControl {
    fn from(value: JvmtiIterationControl) -> Self {
        match value {
            JvmtiIterationControl::Continue => sys::JVMTI_ITERATION_CONTINUE,
            JvmtiIterationControl::Ignore => sys::JVMTI_ITERATION_IGNORE,
            JvmtiIterationControl::Abort => sys::JVMTI_ITERATION_ABORT,
            _ => sys::JVMTI_CONSTANT_UNSUPPORTED,
        }
    }
}