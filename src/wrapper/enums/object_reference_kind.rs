use crate::sys;

#[derive(Debug)]
pub enum JvmtiObjectReferenceKind {
    Class,
    Field,
    ArrayElement,
    ClassLoader,
    Signers,
    ProtectionDomain,
    Interface,
    StaticField,
    ConstantPool,
    Unsupported(sys::jvmtiObjectReferenceKind),
}


impl From<sys::jvmtiObjectReferenceKind> for JvmtiObjectReferenceKind {
    fn from(value: sys::jvmtiObjectReferenceKind) -> Self {
        match value {
            sys::JVMTI_REFERENCE_CLASS => JvmtiObjectReferenceKind::Class,
            sys::JVMTI_REFERENCE_FIELD => JvmtiObjectReferenceKind::Field,
            sys::JVMTI_REFERENCE_ARRAY_ELEMENT => JvmtiObjectReferenceKind::ArrayElement,
            sys::JVMTI_REFERENCE_CLASS_LOADER => JvmtiObjectReferenceKind::ClassLoader,
            sys::JVMTI_REFERENCE_SIGNERS => JvmtiObjectReferenceKind::Signers,
            sys::JVMTI_REFERENCE_PROTECTION_DOMAIN => JvmtiObjectReferenceKind::ProtectionDomain,
            sys::JVMTI_REFERENCE_INTERFACE => JvmtiObjectReferenceKind::Interface,
            sys::JVMTI_REFERENCE_STATIC_FIELD => JvmtiObjectReferenceKind::StaticField,
            sys::JVMTI_REFERENCE_CONSTANT_POOL => JvmtiObjectReferenceKind::ConstantPool,
            _ => JvmtiObjectReferenceKind::Unsupported(value)
        }
    }
}

impl From<JvmtiObjectReferenceKind> for sys::jvmtiObjectReferenceKind {
    fn from(value: JvmtiObjectReferenceKind) -> Self {
        match value {
            JvmtiObjectReferenceKind::Class => sys::JVMTI_REFERENCE_CLASS,
            JvmtiObjectReferenceKind::Field => sys::JVMTI_REFERENCE_FIELD,
            JvmtiObjectReferenceKind::ArrayElement => sys::JVMTI_REFERENCE_ARRAY_ELEMENT,
            JvmtiObjectReferenceKind::ClassLoader => sys::JVMTI_REFERENCE_CLASS_LOADER,
            JvmtiObjectReferenceKind::Signers => sys::JVMTI_REFERENCE_SIGNERS,
            JvmtiObjectReferenceKind::ProtectionDomain => sys::JVMTI_REFERENCE_PROTECTION_DOMAIN,
            JvmtiObjectReferenceKind::Interface => sys::JVMTI_REFERENCE_INTERFACE,
            JvmtiObjectReferenceKind::StaticField => sys::JVMTI_REFERENCE_STATIC_FIELD,
            JvmtiObjectReferenceKind::ConstantPool => sys::JVMTI_REFERENCE_CONSTANT_POOL,
            _ => sys::JVMTI_CONSTANT_UNSUPPORTED,
        }
    }
}