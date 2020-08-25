use crate::sys;

#[derive(Debug)]
pub enum JvmtiPrimitiveType {
    Boolean,
    Byte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Unsupported(sys::jvmtiPrimitiveType),
}


impl From<sys::jvmtiPrimitiveType> for JvmtiPrimitiveType {
    fn from(value: sys::jvmtiPrimitiveType) -> Self {
        match value {
            sys::JVMTI_PRIMITIVE_TYPE_BOOLEAN => JvmtiPrimitiveType::Boolean,
            sys::JVMTI_PRIMITIVE_TYPE_BYTE => JvmtiPrimitiveType::Byte,
            sys::JVMTI_PRIMITIVE_TYPE_CHAR => JvmtiPrimitiveType::Char,
            sys::JVMTI_PRIMITIVE_TYPE_SHORT => JvmtiPrimitiveType::Short,
            sys::JVMTI_PRIMITIVE_TYPE_INT => JvmtiPrimitiveType::Int,
            sys::JVMTI_PRIMITIVE_TYPE_LONG => JvmtiPrimitiveType::Long,
            sys::JVMTI_PRIMITIVE_TYPE_FLOAT => JvmtiPrimitiveType::Float,
            sys::JVMTI_PRIMITIVE_TYPE_DOUBLE => JvmtiPrimitiveType::Double,
            _ => JvmtiPrimitiveType::Unsupported(value)
        }
    }
}

impl From<JvmtiPrimitiveType> for sys::jvmtiPrimitiveType {
    fn from(value: JvmtiPrimitiveType) -> Self {
        match value {
            JvmtiPrimitiveType::Boolean => sys::JVMTI_PRIMITIVE_TYPE_BOOLEAN,
            JvmtiPrimitiveType::Byte => sys::JVMTI_PRIMITIVE_TYPE_BYTE,
            JvmtiPrimitiveType::Char => sys::JVMTI_PRIMITIVE_TYPE_CHAR,
            JvmtiPrimitiveType::Short => sys::JVMTI_PRIMITIVE_TYPE_SHORT,
            JvmtiPrimitiveType::Int => sys::JVMTI_PRIMITIVE_TYPE_INT,
            JvmtiPrimitiveType::Long => sys::JVMTI_PRIMITIVE_TYPE_LONG,
            JvmtiPrimitiveType::Float => sys::JVMTI_PRIMITIVE_TYPE_FLOAT,
            JvmtiPrimitiveType::Double => sys::JVMTI_PRIMITIVE_TYPE_DOUBLE,
            _ => sys::JVMTI_CONSTANT_UNSUPPORTED,
        }
    }
}