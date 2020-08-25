use crate::sys;

#[derive(Debug)]
pub enum JvmtiHeapObjectFilter {
    Tagged,
    Untagged,
    Either,
    Unsupported(sys::jvmtiHeapObjectFilter),
}


impl From<sys::jvmtiHeapObjectFilter> for JvmtiHeapObjectFilter {
    fn from(value: sys::jvmtiHeapObjectFilter) -> Self {
        match value {
            sys::JVMTI_HEAP_OBJECT_TAGGED => JvmtiHeapObjectFilter::Tagged,
            sys::JVMTI_HEAP_OBJECT_UNTAGGED => JvmtiHeapObjectFilter::Untagged,
            sys::JVMTI_HEAP_OBJECT_EITHER => JvmtiHeapObjectFilter::Either,
            _ => JvmtiHeapObjectFilter::Unsupported(value)
        }
    }
}

impl From<JvmtiHeapObjectFilter> for sys::jvmtiHeapObjectFilter {
    fn from(value: JvmtiHeapObjectFilter) -> Self {
        match value {
            JvmtiHeapObjectFilter::Tagged => sys::JVMTI_HEAP_OBJECT_TAGGED,
            JvmtiHeapObjectFilter::Untagged => sys::JVMTI_HEAP_OBJECT_UNTAGGED,
            JvmtiHeapObjectFilter::Either => sys::JVMTI_HEAP_OBJECT_EITHER,
            _ => sys::JVMTI_CONSTANT_UNSUPPORTED,
        }
    }
}