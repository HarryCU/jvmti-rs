use crate::sys;

#[derive(Debug)]
pub enum JvmtiParamKind {
    In,
    InPtr,
    InBuf,
    AllocBuf,
    AllocAllocBuf,
    Out,
    OutBuf,
    Unsupported(sys::jvmtiParamKind),
}


impl From<sys::jvmtiParamKind> for JvmtiParamKind {
    fn from(value: sys::jvmtiParamKind) -> Self {
        match value {
            sys::JVMTI_KIND_IN => JvmtiParamKind::In,
            sys::JVMTI_KIND_IN_PTR => JvmtiParamKind::InPtr,
            sys::JVMTI_KIND_IN_BUF => JvmtiParamKind::InBuf,
            sys::JVMTI_KIND_ALLOC_BUF => JvmtiParamKind::AllocBuf,
            sys::JVMTI_KIND_ALLOC_ALLOC_BUF => JvmtiParamKind::AllocAllocBuf,
            sys::JVMTI_KIND_OUT => JvmtiParamKind::Out,
            sys::JVMTI_KIND_OUT_BUF => JvmtiParamKind::OutBuf,
            _ => JvmtiParamKind::Unsupported(value)
        }
    }
}

impl From<JvmtiParamKind> for sys::jvmtiParamKind {
    fn from(value: JvmtiParamKind) -> Self {
        match value {
            JvmtiParamKind::In => sys::JVMTI_KIND_IN,
            JvmtiParamKind::InPtr => sys::JVMTI_KIND_IN_PTR,
            JvmtiParamKind::InBuf => sys::JVMTI_KIND_IN_BUF,
            JvmtiParamKind::AllocBuf => sys::JVMTI_KIND_ALLOC_BUF,
            JvmtiParamKind::AllocAllocBuf => sys::JVMTI_KIND_ALLOC_ALLOC_BUF,
            JvmtiParamKind::Out => sys::JVMTI_KIND_OUT,
            JvmtiParamKind::OutBuf => sys::JVMTI_KIND_OUT_BUF,
            _ => sys::JVMTI_CONSTANT_UNSUPPORTED,
        }
    }
}