use crate::sys;

#[derive(Debug)]
pub enum JvmtiJavaLangThreadState {
    Mask,
    New,
    Terminated,
    Runnable,
    Blocked,
    Waiting,
    TimedWaiting,
    Unsupported(sys::jvmtiJavaLangThreadState),
}

impl From<sys::jvmtiJavaLangThreadState> for JvmtiJavaLangThreadState {
    fn from(value: sys::jvmtiJavaLangThreadState) -> Self {
        match value {
            sys::JVMTI_JAVA_LANG_THREAD_STATE_MASK => JvmtiJavaLangThreadState::Mask,
            sys::JVMTI_JAVA_LANG_THREAD_STATE_NEW => JvmtiJavaLangThreadState::New,
            sys::JVMTI_JAVA_LANG_THREAD_STATE_TERMINATED => JvmtiJavaLangThreadState::Terminated,
            sys::JVMTI_JAVA_LANG_THREAD_STATE_RUNNABLE => JvmtiJavaLangThreadState::Runnable,
            sys::JVMTI_JAVA_LANG_THREAD_STATE_BLOCKED => JvmtiJavaLangThreadState::Blocked,
            sys::JVMTI_JAVA_LANG_THREAD_STATE_WAITING => JvmtiJavaLangThreadState::Waiting,
            sys::JVMTI_JAVA_LANG_THREAD_STATE_TIMED_WAITING => JvmtiJavaLangThreadState::TimedWaiting,
            _ => JvmtiJavaLangThreadState::Unsupported(value)
        }
    }
}

impl From<JvmtiJavaLangThreadState> for sys::jvmtiJavaLangThreadState {
    fn from(value: JvmtiJavaLangThreadState) -> Self {
        match value {
            JvmtiJavaLangThreadState::Mask => sys::JVMTI_JAVA_LANG_THREAD_STATE_MASK,
            JvmtiJavaLangThreadState::New => sys::JVMTI_JAVA_LANG_THREAD_STATE_NEW,
            JvmtiJavaLangThreadState::Terminated => sys::JVMTI_JAVA_LANG_THREAD_STATE_TERMINATED,
            JvmtiJavaLangThreadState::Runnable => sys::JVMTI_JAVA_LANG_THREAD_STATE_RUNNABLE,
            JvmtiJavaLangThreadState::Blocked => sys::JVMTI_JAVA_LANG_THREAD_STATE_BLOCKED,
            JvmtiJavaLangThreadState::Waiting => sys::JVMTI_JAVA_LANG_THREAD_STATE_WAITING,
            JvmtiJavaLangThreadState::TimedWaiting => sys::JVMTI_JAVA_LANG_THREAD_STATE_TIMED_WAITING,
            _ => sys::JVMTI_CONSTANT_UNSUPPORTED,
        }
    }
}