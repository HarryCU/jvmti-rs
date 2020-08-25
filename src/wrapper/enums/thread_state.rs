use crate::sys;

#[derive(Debug)]
pub enum JvmtiThreadState {
    Alive,
    Terminated,
    Runnable,
    BlockedOnMonitorEnter,
    Waiting,
    WaitingIndefinitely,
    WaitingWithTimeout,
    Sleeping,
    InObjectWait,
    Parked,
    Suspended,
    Interrupted,
    InNative,
    Vendor1,
    Vendor2,
    Vendor3,
    Unsupported(sys::jvmtiThreadState),
}

impl From<sys::jvmtiThreadState> for JvmtiThreadState {
    fn from(value: sys::jvmtiThreadState) -> Self {
        match value {
            sys::JVMTI_THREAD_STATE_ALIVE => JvmtiThreadState::Alive,
            sys::JVMTI_THREAD_STATE_TERMINATED => JvmtiThreadState::Terminated,
            sys::JVMTI_THREAD_STATE_RUNNABLE => JvmtiThreadState::Runnable,
            sys::JVMTI_THREAD_STATE_BLOCKED_ON_MONITOR_ENTER => JvmtiThreadState::BlockedOnMonitorEnter,
            sys::JVMTI_THREAD_STATE_WAITING => JvmtiThreadState::Waiting,
            sys::JVMTI_THREAD_STATE_WAITING_INDEFINITELY => JvmtiThreadState::WaitingIndefinitely,
            sys::JVMTI_THREAD_STATE_WAITING_WITH_TIMEOUT => JvmtiThreadState::WaitingWithTimeout,
            sys::JVMTI_THREAD_STATE_SLEEPING => JvmtiThreadState::Sleeping,
            sys::JVMTI_THREAD_STATE_IN_OBJECT_WAIT => JvmtiThreadState::InObjectWait,
            sys::JVMTI_THREAD_STATE_PARKED => JvmtiThreadState::Parked,
            sys::JVMTI_THREAD_STATE_SUSPENDED => JvmtiThreadState::Suspended,
            sys::JVMTI_THREAD_STATE_INTERRUPTED => JvmtiThreadState::Interrupted,
            sys::JVMTI_THREAD_STATE_IN_NATIVE => JvmtiThreadState::InNative,
            sys::JVMTI_THREAD_STATE_VENDOR_1 => JvmtiThreadState::Vendor1,
            sys::JVMTI_THREAD_STATE_VENDOR_2 => JvmtiThreadState::Vendor2,
            sys::JVMTI_THREAD_STATE_VENDOR_3 => JvmtiThreadState::Vendor3,
            _ => JvmtiThreadState::Unsupported(value)
        }
    }
}

impl From<JvmtiThreadState> for sys::jvmtiThreadState {
    fn from(value: JvmtiThreadState) -> Self {
        match value {
            JvmtiThreadState::Alive => sys::JVMTI_THREAD_STATE_ALIVE,
            JvmtiThreadState::Terminated => sys::JVMTI_THREAD_STATE_TERMINATED,
            JvmtiThreadState::Runnable => sys::JVMTI_THREAD_STATE_RUNNABLE,
            JvmtiThreadState::BlockedOnMonitorEnter => sys::JVMTI_THREAD_STATE_BLOCKED_ON_MONITOR_ENTER,
            JvmtiThreadState::Waiting => sys::JVMTI_THREAD_STATE_WAITING,
            JvmtiThreadState::WaitingIndefinitely => sys::JVMTI_THREAD_STATE_WAITING_INDEFINITELY,
            JvmtiThreadState::WaitingWithTimeout => sys::JVMTI_THREAD_STATE_WAITING_WITH_TIMEOUT,
            JvmtiThreadState::Sleeping => sys::JVMTI_THREAD_STATE_SLEEPING,
            JvmtiThreadState::InObjectWait => sys::JVMTI_THREAD_STATE_IN_OBJECT_WAIT,
            JvmtiThreadState::Parked => sys::JVMTI_THREAD_STATE_PARKED,
            JvmtiThreadState::Suspended => sys::JVMTI_THREAD_STATE_SUSPENDED,
            JvmtiThreadState::Interrupted => sys::JVMTI_THREAD_STATE_INTERRUPTED,
            JvmtiThreadState::InNative => sys::JVMTI_THREAD_STATE_IN_NATIVE,
            JvmtiThreadState::Vendor1 => sys::JVMTI_THREAD_STATE_VENDOR_1,
            JvmtiThreadState::Vendor2 => sys::JVMTI_THREAD_STATE_VENDOR_2,
            JvmtiThreadState::Vendor3 => sys::JVMTI_THREAD_STATE_VENDOR_3,
            _ => sys::JVMTI_CONSTANT_UNSUPPORTED,
        }
    }
}