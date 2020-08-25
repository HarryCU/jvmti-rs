use crate::sys;

#[derive(Debug)]
pub enum JvmtiHeapRootKind {
    JniGlobal,
    SystemClass,
    Monitor,
    StackLocal,
    JniLocal,
    Thread,
    Other,
    Unsupported(sys::jvmtiHeapRootKind),
}


impl From<sys::jvmtiHeapRootKind> for JvmtiHeapRootKind {
    fn from(value: sys::jvmtiHeapRootKind) -> Self {
        match value {
            sys::JVMTI_HEAP_ROOT_JNI_GLOBAL => JvmtiHeapRootKind::JniGlobal,
            sys::JVMTI_HEAP_ROOT_SYSTEM_CLASS => JvmtiHeapRootKind::SystemClass,
            sys::JVMTI_HEAP_ROOT_MONITOR => JvmtiHeapRootKind::Monitor,
            sys::JVMTI_HEAP_ROOT_STACK_LOCAL => JvmtiHeapRootKind::StackLocal,
            sys::JVMTI_HEAP_ROOT_JNI_LOCAL => JvmtiHeapRootKind::JniLocal,
            sys::JVMTI_HEAP_ROOT_THREAD => JvmtiHeapRootKind::Thread,
            sys::JVMTI_HEAP_ROOT_OTHER => JvmtiHeapRootKind::Other,
            _ => JvmtiHeapRootKind::Unsupported(value)
        }
    }
}

impl From<JvmtiHeapRootKind> for sys::jvmtiHeapRootKind {
    fn from(value: JvmtiHeapRootKind) -> Self {
        match value {
            JvmtiHeapRootKind::JniGlobal => sys::JVMTI_HEAP_ROOT_JNI_GLOBAL,
            JvmtiHeapRootKind::SystemClass => sys::JVMTI_HEAP_ROOT_SYSTEM_CLASS,
            JvmtiHeapRootKind::Monitor => sys::JVMTI_HEAP_ROOT_MONITOR,
            JvmtiHeapRootKind::StackLocal => sys::JVMTI_HEAP_ROOT_STACK_LOCAL,
            JvmtiHeapRootKind::JniLocal => sys::JVMTI_HEAP_ROOT_JNI_LOCAL,
            JvmtiHeapRootKind::Thread => sys::JVMTI_HEAP_ROOT_THREAD,
            JvmtiHeapRootKind::Other => sys::JVMTI_HEAP_ROOT_OTHER,
            _ => sys::JVMTI_CONSTANT_UNSUPPORTED,
        }
    }
}