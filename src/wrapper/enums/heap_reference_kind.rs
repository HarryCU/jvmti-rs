use crate::sys;

#[derive(Debug)]
pub enum JvmtiHeapReferenceKind {
    Class,
    Field,
    ArrayElement,
    ClassLoader,
    Signers,
    ProtectionDomain,
    Interface,
    StaticField,
    ConstantPool,
    Superclass,
    JniGlobal,
    SystemClass,
    Monitor,
    StackLocal,
    JniLocal,
    Thread,
    Other,
    Unsupported(sys::jvmtiHeapReferenceKind),
}


impl From<sys::jvmtiHeapReferenceKind> for JvmtiHeapReferenceKind {
    fn from(value: sys::jvmtiHeapReferenceKind) -> Self {
        match value {
            sys::JVMTI_HEAP_REFERENCE_CLASS => JvmtiHeapReferenceKind::Class,
            sys::JVMTI_HEAP_REFERENCE_FIELD => JvmtiHeapReferenceKind::Field,
            sys::JVMTI_HEAP_REFERENCE_ARRAY_ELEMENT => JvmtiHeapReferenceKind::ArrayElement,
            sys::JVMTI_HEAP_REFERENCE_CLASS_LOADER => JvmtiHeapReferenceKind::ClassLoader,
            sys::JVMTI_HEAP_REFERENCE_SIGNERS => JvmtiHeapReferenceKind::Signers,
            sys::JVMTI_HEAP_REFERENCE_PROTECTION_DOMAIN => JvmtiHeapReferenceKind::ProtectionDomain,
            sys::JVMTI_HEAP_REFERENCE_INTERFACE => JvmtiHeapReferenceKind::Interface,
            sys::JVMTI_HEAP_REFERENCE_STATIC_FIELD => JvmtiHeapReferenceKind::StaticField,
            sys::JVMTI_HEAP_REFERENCE_CONSTANT_POOL => JvmtiHeapReferenceKind::ConstantPool,
            sys::JVMTI_HEAP_REFERENCE_SUPERCLASS => JvmtiHeapReferenceKind::Superclass,
            sys::JVMTI_HEAP_REFERENCE_JNI_GLOBAL => JvmtiHeapReferenceKind::JniGlobal,
            sys::JVMTI_HEAP_REFERENCE_SYSTEM_CLASS => JvmtiHeapReferenceKind::SystemClass,
            sys::JVMTI_HEAP_REFERENCE_MONITOR => JvmtiHeapReferenceKind::Monitor,
            sys::JVMTI_HEAP_REFERENCE_STACK_LOCAL => JvmtiHeapReferenceKind::StackLocal,
            sys::JVMTI_HEAP_REFERENCE_JNI_LOCAL => JvmtiHeapReferenceKind::JniLocal,
            sys::JVMTI_HEAP_REFERENCE_THREAD => JvmtiHeapReferenceKind::Thread,
            sys::JVMTI_HEAP_REFERENCE_OTHER => JvmtiHeapReferenceKind::Other,
            _ => JvmtiHeapReferenceKind::Unsupported(value)
        }
    }
}

impl From<JvmtiHeapReferenceKind> for sys::jvmtiHeapReferenceKind {
    fn from(value: JvmtiHeapReferenceKind) -> Self {
        match value {
            JvmtiHeapReferenceKind::Class => sys::JVMTI_HEAP_REFERENCE_CLASS,
            JvmtiHeapReferenceKind::Field => sys::JVMTI_HEAP_REFERENCE_FIELD,
            JvmtiHeapReferenceKind::ArrayElement => sys::JVMTI_HEAP_REFERENCE_ARRAY_ELEMENT,
            JvmtiHeapReferenceKind::ClassLoader => sys::JVMTI_HEAP_REFERENCE_CLASS_LOADER,
            JvmtiHeapReferenceKind::Signers => sys::JVMTI_HEAP_REFERENCE_SIGNERS,
            JvmtiHeapReferenceKind::ProtectionDomain => sys::JVMTI_HEAP_REFERENCE_PROTECTION_DOMAIN,
            JvmtiHeapReferenceKind::Interface => sys::JVMTI_HEAP_REFERENCE_INTERFACE,
            JvmtiHeapReferenceKind::StaticField => sys::JVMTI_HEAP_REFERENCE_STATIC_FIELD,
            JvmtiHeapReferenceKind::ConstantPool => sys::JVMTI_HEAP_REFERENCE_CONSTANT_POOL,
            JvmtiHeapReferenceKind::Superclass => sys::JVMTI_HEAP_REFERENCE_SUPERCLASS,
            JvmtiHeapReferenceKind::JniGlobal => sys::JVMTI_HEAP_REFERENCE_JNI_GLOBAL,
            JvmtiHeapReferenceKind::SystemClass => sys::JVMTI_HEAP_REFERENCE_SYSTEM_CLASS,
            JvmtiHeapReferenceKind::Monitor => sys::JVMTI_HEAP_REFERENCE_MONITOR,
            JvmtiHeapReferenceKind::StackLocal => sys::JVMTI_HEAP_REFERENCE_STACK_LOCAL,
            JvmtiHeapReferenceKind::JniLocal => sys::JVMTI_HEAP_REFERENCE_JNI_LOCAL,
            JvmtiHeapReferenceKind::Thread => sys::JVMTI_HEAP_REFERENCE_THREAD,
            JvmtiHeapReferenceKind::Other => sys::JVMTI_HEAP_REFERENCE_OTHER,
            _ => sys::JVMTI_CONSTANT_UNSUPPORTED,
        }
    }
}