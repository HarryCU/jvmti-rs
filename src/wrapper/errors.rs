#![allow(missing_docs)]

use error_chain::*;

use crate::sys;
use crate::sys::jvmtiError;
use crate::wrapper::JvmtiError;
use cesu8::Cesu8DecodingError;

// see https://github.com/jni-rs/jni-rs/blob/master/src/wrapper/errors.rs
error_chain! {
    foreign_links {
    }

    errors {
        JVMTIEnvMethodNotFound(name: &'static str) {
            description("Method pointer null in JVMTIEnv")
            display("JVMTIEnv null method pointer for {}", name)
        }
        NullPtr(context: &'static str) {
            description("null pointer")
            display("null pointer in {}", context)
        }
        JavaVMMethodNotFound(name: &'static str) {
            description("Method pointer null in JavaVM")
            display("JavaVM null method pointer for {}", name)
        }
        NullDeref(context: &'static str) {
            description("null pointer deref")
            display("null pointer deref in {}", context)
        }
        KnowJvmtiError(error: JvmtiError) {
            description("JVMTI know error")
            display("{}", error)
        }
        Other(error: sys::jvmtiError) {
            description("JVMTI unknown error")
            display("jvmtiError => {}", error)
        }
    }
}

unsafe impl Sync for Error {}

pub fn jvmti_error_code_to_result(code: sys::jvmtiError) -> Result<()> {
    match code {
        sys::JVMTI_ERROR_NONE => Ok(()),
        code if KNOW_JVMTI_ERROR.contains(&code) => Err(Error::from(ErrorKind::KnowJvmtiError(JvmtiError::from(code)))),
        _ => Err(Error::from(ErrorKind::Other(code))),
    }
}

lazy_static! {
    static ref KNOW_JVMTI_ERROR: Vec<jvmtiError> = {
        return vec![
            sys::JVMTI_ERROR_INVALID_THREAD,
            sys::JVMTI_ERROR_INVALID_THREAD_GROUP,
            sys::JVMTI_ERROR_INVALID_PRIORITY,
            sys::JVMTI_ERROR_THREAD_NOT_SUSPENDED,
            sys::JVMTI_ERROR_THREAD_SUSPENDED,
            sys::JVMTI_ERROR_THREAD_NOT_ALIVE,
            sys::JVMTI_ERROR_INVALID_OBJECT,
            sys::JVMTI_ERROR_INVALID_CLASS,
            sys::JVMTI_ERROR_CLASS_NOT_PREPARED,
            sys::JVMTI_ERROR_INVALID_METHODID,
            sys::JVMTI_ERROR_INVALID_LOCATION,
            sys::JVMTI_ERROR_INVALID_FIELDID,
            sys::JVMTI_ERROR_NO_MORE_FRAMES,
            sys::JVMTI_ERROR_OPAQUE_FRAME,
            sys::JVMTI_ERROR_TYPE_MISMATCH,
            sys::JVMTI_ERROR_INVALID_SLOT,
            sys::JVMTI_ERROR_DUPLICATE,
            sys::JVMTI_ERROR_NOT_FOUND,
            sys::JVMTI_ERROR_INVALID_MONITOR,
            sys::JVMTI_ERROR_NOT_MONITOR_OWNER,
            sys::JVMTI_ERROR_INTERRUPT,
            sys::JVMTI_ERROR_INVALID_CLASS_FORMAT,
            sys::JVMTI_ERROR_CIRCULAR_CLASS_DEFINITION,
            sys::JVMTI_ERROR_FAILS_VERIFICATION,
            sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_ADDED,
            sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED,
            sys::JVMTI_ERROR_INVALID_TYPESTATE,
            sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_HIERARCHY_CHANGED,
            sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_DELETED,
            sys::JVMTI_ERROR_UNSUPPORTED_VERSION,
            sys::JVMTI_ERROR_NAMES_DONT_MATCH,
            sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_MODIFIERS_CHANGED,
            sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_MODIFIERS_CHANGED,
            sys::JVMTI_ERROR_UNMODIFIABLE_CLASS,
            sys::JVMTI_ERROR_NOT_AVAILABLE,
            sys::JVMTI_ERROR_MUST_POSSESS_CAPABILITY,
            sys::JVMTI_ERROR_NULL_POINTER,
            sys::JVMTI_ERROR_ABSENT_INFORMATION,
            sys::JVMTI_ERROR_INVALID_EVENT_TYPE,
            sys::JVMTI_ERROR_ILLEGAL_ARGUMENT,
            sys::JVMTI_ERROR_NATIVE_METHOD,
            sys::JVMTI_ERROR_CLASS_LOADER_UNSUPPORTED,
            sys::JVMTI_ERROR_OUT_OF_MEMORY,
            sys::JVMTI_ERROR_ACCESS_DENIED,
            sys::JVMTI_ERROR_WRONG_PHASE,
            sys::JVMTI_ERROR_INTERNAL,
            sys::JVMTI_ERROR_UNATTACHED_THREAD,
            sys::JVMTI_ERROR_INVALID_ENVIRONMENT,
        ];
    };
}
