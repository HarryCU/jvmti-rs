use crate::sys;

#[derive(Debug)]
pub enum JvmtiHeapFilter {
    Tagged,
    Untagged,
    ClassTagged,
    ClassUntagged,
    Unsupported(sys::jvmtiHeapFilter),
}


impl From<sys::jvmtiHeapFilter> for JvmtiHeapFilter {
    fn from(value: sys::jvmtiHeapFilter) -> Self {
        match value {
            sys::JVMTI_HEAP_FILTER_TAGGED => JvmtiHeapFilter::Tagged,
            sys::JVMTI_HEAP_FILTER_UNTAGGED => JvmtiHeapFilter::Untagged,
            sys::JVMTI_HEAP_FILTER_CLASS_TAGGED => JvmtiHeapFilter::ClassTagged,
            sys::JVMTI_HEAP_FILTER_CLASS_UNTAGGED => JvmtiHeapFilter::ClassUntagged,
            _ => JvmtiHeapFilter::Unsupported(value)
        }
    }
}

impl From<JvmtiHeapFilter> for sys::jvmtiHeapFilter {
    fn from(value: JvmtiHeapFilter) -> Self {
        match value {
            JvmtiHeapFilter::Tagged => sys::JVMTI_HEAP_FILTER_TAGGED,
            JvmtiHeapFilter::Untagged => sys::JVMTI_HEAP_FILTER_UNTAGGED,
            JvmtiHeapFilter::ClassTagged => sys::JVMTI_HEAP_FILTER_CLASS_TAGGED,
            JvmtiHeapFilter::ClassUntagged => sys::JVMTI_HEAP_FILTER_CLASS_UNTAGGED,
            _ => sys::JVMTI_CONSTANT_UNSUPPORTED,
        }
    }
}