use crate::sys;

#[derive(Debug)]
pub enum JvmtiParamTypes {
    JByte,
    JChar,
    JShort,
    JInt,
    JLong,
    JFloat,
    JDouble,
    JBoolean,
    JObject,
    JThread,
    JClass,
    JValue,
    JFieldId,
    JMethodId,
    CChar,
    CVoid,
    JniEnv,
    Unsupported(sys::jvmtiParamTypes),
}


impl From<sys::jvmtiParamTypes> for JvmtiParamTypes {
    fn from(value: sys::jvmtiParamTypes) -> Self {
        match value {
            sys::JVMTI_TYPE_JBYTE => JvmtiParamTypes::JByte,
            sys::JVMTI_TYPE_JCHAR => JvmtiParamTypes::JChar,
            sys::JVMTI_TYPE_JSHORT => JvmtiParamTypes::JShort,
            sys::JVMTI_TYPE_JINT => JvmtiParamTypes::JInt,
            sys::JVMTI_TYPE_JLONG => JvmtiParamTypes::JLong,
            sys::JVMTI_TYPE_JFLOAT => JvmtiParamTypes::JFloat,
            sys::JVMTI_TYPE_JDOUBLE => JvmtiParamTypes::JDouble,
            sys::JVMTI_TYPE_JBOOLEAN => JvmtiParamTypes::JBoolean,
            sys::JVMTI_TYPE_JOBJECT => JvmtiParamTypes::JObject,
            sys::JVMTI_TYPE_JTHREAD => JvmtiParamTypes::JThread,
            sys::JVMTI_TYPE_JCLASS => JvmtiParamTypes::JClass,
            sys::JVMTI_TYPE_JVALUE => JvmtiParamTypes::JValue,
            sys::JVMTI_TYPE_JFIELDID => JvmtiParamTypes::JFieldId,
            sys::JVMTI_TYPE_JMETHODID => JvmtiParamTypes::JMethodId,
            sys::JVMTI_TYPE_CCHAR => JvmtiParamTypes::CChar,
            sys::JVMTI_TYPE_CVOID => JvmtiParamTypes::CVoid,
            sys::JVMTI_TYPE_JNIENV => JvmtiParamTypes::JniEnv,
            _ => JvmtiParamTypes::Unsupported(value)
        }
    }
}

impl From<JvmtiParamTypes> for sys::jvmtiParamTypes {
    fn from(value: JvmtiParamTypes) -> Self {
        match value {
            JvmtiParamTypes::JByte => sys::JVMTI_TYPE_JBYTE,
            JvmtiParamTypes::JChar => sys::JVMTI_TYPE_JCHAR,
            JvmtiParamTypes::JShort => sys::JVMTI_TYPE_JSHORT,
            JvmtiParamTypes::JInt => sys::JVMTI_TYPE_JINT,
            JvmtiParamTypes::JLong => sys::JVMTI_TYPE_JLONG,
            JvmtiParamTypes::JFloat => sys::JVMTI_TYPE_JFLOAT,
            JvmtiParamTypes::JDouble => sys::JVMTI_TYPE_JDOUBLE,
            JvmtiParamTypes::JBoolean => sys::JVMTI_TYPE_JBOOLEAN,
            JvmtiParamTypes::JObject => sys::JVMTI_TYPE_JOBJECT,
            JvmtiParamTypes::JThread => sys::JVMTI_TYPE_JTHREAD,
            JvmtiParamTypes::JClass => sys::JVMTI_TYPE_JCLASS,
            JvmtiParamTypes::JValue => sys::JVMTI_TYPE_JVALUE,
            JvmtiParamTypes::JFieldId => sys::JVMTI_TYPE_JFIELDID,
            JvmtiParamTypes::JMethodId => sys::JVMTI_TYPE_JMETHODID,
            JvmtiParamTypes::CChar => sys::JVMTI_TYPE_CCHAR,
            JvmtiParamTypes::CVoid => sys::JVMTI_TYPE_CVOID,
            JvmtiParamTypes::JniEnv => sys::JVMTI_TYPE_JNIENV,
            _ => sys::JVMTI_CONSTANT_UNSUPPORTED,
        }
    }
}