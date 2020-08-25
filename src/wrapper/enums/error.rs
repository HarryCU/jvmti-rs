use crate::sys;
use thiserror::Error;

/// see https://github.com/rel-eng/rvmti/blob/master/src/rvmti.rs#L527
///
#[derive(Error, Debug)]
pub enum JvmtiError {
    #[error("Empty argument")]
    EmptyArgument,
    #[error("Invalid thread")]
    InvalidThread,
    #[error("Invalid thread group")]
    InvalidThreadGroup,
    #[error("Invalid priority")]
    InvalidPriority,
    #[error("Thread is not suspended")]
    ThreadNotSuspended,
    #[error("Thread is already suspended")]
    ThreadSuspended,
    #[error("Thread is not alive")]
    ThreadNotAlive,
    #[error("Invalid object")]
    InvalidObject,
    #[error("Invalid class")]
    InvalidClass,
    #[error("The class is not prepared yet")]
    ClassNotPrepared,
    #[error("Invalid method id")]
    InvalidMethodId,
    #[error("Invalid location")]
    InvalidLocation,
    #[error("Invalid field id")]
    InvalidFieldId,
    #[error("There are no more stack frames")]
    NoMoreFrames,
    #[error("No information is available about the stack frame")]
    OpaqueFrame,
    #[error("Variable type mismatch")]
    TypeMismatch,
    #[error("Invalid slot")]
    InvalidSlot,
    #[error("The item is already set")]
    Duplicate,
    #[error("Element is not found")]
    NotFound,
    #[error("Invalid raw monitor")]
    InvalidMonitor,
    #[error("The raw monitor is not owned by this thread")]
    NotMonitorOwner,
    #[error("The call has been interrupted")]
    Interrupt,
    #[error("Malformed class file")]
    InvalidClassFormat,
    #[error("Circular class definition")]
    CircularClassDefinition,
    #[error("The class fails verification")]
    FailsVerification,
    #[error("Class redefinition not possible, method addition is unsupported")]
    UnsupportedRedefinitionMethodAdded,
    #[error("Class redefinition not possible, field change is unsupported")]
    UnsupportedRedefinitionSchemaChanged,
    #[error("The thread state is inconsistent due to it having been modified")]
    InvalidTypeState,
    #[error("Class redefinition not possible, class hierarchy change is unsupported")]
    UnsupportedRedefinitionHierarchyChanged,
    #[error("Class redefinition not possible, method deletion is unsupported")]
    UnsupportedRedefinitionMethodDeleted,
    #[error("Class file version is unsupported")]
    UnsupportedVersion,
    #[error("Class names do not match")]
    NamesDontMatch,
    #[error("Class redefinition not possible, class modifiers change is unsupported")]
    UnsupportedRedefinitionClassModifiersChanged,
    #[error("Class redefinition not possible, method modifiers change is unsupported")]
    UnsupportedRedefinitionMethodModifiersChanged,
    #[error("The class is unmodifiable")]
    UnmodifiableClass,
    #[error("The functionality is not available")]
    NotAvaliable,
    #[error("This environment does not possess the required capability")]
    MustPosessCapability,
    #[error("Unexpected null pointer")]
    NullPointer,
    #[error("Information is not available")]
    AbsentInformation,
    #[error("Invalid event type")]
    InvalidEventType,
    #[error("Illegal argument")]
    IllegalArgument,
    #[error("Information is not available for native method")]
    NativeMethod,
    #[error("This class loader does not support the requested operation")]
    ClassLoaderUnsupported,
    #[error("Out of memory")]
    OutOfMemory,
    #[error("Access denied")]
    AccessDenied,
    #[error("The functionality is not available in the current phase")]
    WrongPhase,
    #[error("Unexpected internal error")]
    Internal,
    #[error("The thread is not attached to the virtual machine")]
    UnattachedThread,
    #[error("Invalid environment")]
    InvalidEnvironment,
    #[error("Unsupported JVMTI error code: {0}")]
    UnsupportedError(sys::jvmtiError),
}

impl From<sys::jvmtiError> for JvmtiError {
    fn from(error: sys::jvmtiError) -> Self {
        match error {
            sys::JVMTI_ERROR_INVALID_THREAD => JvmtiError::InvalidThread,
            sys::JVMTI_ERROR_INVALID_THREAD_GROUP => JvmtiError::InvalidThreadGroup,
            sys::JVMTI_ERROR_INVALID_PRIORITY => JvmtiError::InvalidPriority,
            sys::JVMTI_ERROR_THREAD_NOT_SUSPENDED => JvmtiError::ThreadNotSuspended,
            sys::JVMTI_ERROR_THREAD_SUSPENDED => JvmtiError::ThreadSuspended,
            sys::JVMTI_ERROR_THREAD_NOT_ALIVE => JvmtiError::ThreadNotAlive,
            sys::JVMTI_ERROR_INVALID_OBJECT => JvmtiError::InvalidObject,
            sys::JVMTI_ERROR_INVALID_CLASS => JvmtiError::InvalidClass,
            sys::JVMTI_ERROR_CLASS_NOT_PREPARED => JvmtiError::ClassNotPrepared,
            sys::JVMTI_ERROR_INVALID_METHODID => JvmtiError::InvalidMethodId,
            sys::JVMTI_ERROR_INVALID_LOCATION => JvmtiError::InvalidLocation,
            sys::JVMTI_ERROR_INVALID_FIELDID => JvmtiError::InvalidFieldId,
            sys::JVMTI_ERROR_NO_MORE_FRAMES => JvmtiError::NoMoreFrames,
            sys::JVMTI_ERROR_OPAQUE_FRAME => JvmtiError::OpaqueFrame,
            sys::JVMTI_ERROR_TYPE_MISMATCH => JvmtiError::TypeMismatch,
            sys::JVMTI_ERROR_INVALID_SLOT => JvmtiError::InvalidSlot,
            sys::JVMTI_ERROR_DUPLICATE => JvmtiError::Duplicate,
            sys::JVMTI_ERROR_NOT_FOUND => JvmtiError::NotFound,
            sys::JVMTI_ERROR_INVALID_MONITOR => JvmtiError::InvalidMonitor,
            sys::JVMTI_ERROR_NOT_MONITOR_OWNER => JvmtiError::NotMonitorOwner,
            sys::JVMTI_ERROR_INTERRUPT => JvmtiError::Interrupt,
            sys::JVMTI_ERROR_INVALID_CLASS_FORMAT => JvmtiError::InvalidClassFormat,
            sys::JVMTI_ERROR_CIRCULAR_CLASS_DEFINITION => JvmtiError::CircularClassDefinition,
            sys::JVMTI_ERROR_FAILS_VERIFICATION => JvmtiError::FailsVerification,
            sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_ADDED => JvmtiError::UnsupportedRedefinitionMethodAdded,
            sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED => JvmtiError::UnsupportedRedefinitionSchemaChanged,
            sys::JVMTI_ERROR_INVALID_TYPESTATE => JvmtiError::InvalidTypeState,
            sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_HIERARCHY_CHANGED => JvmtiError::UnsupportedRedefinitionHierarchyChanged,
            sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_DELETED => JvmtiError::UnsupportedRedefinitionMethodDeleted,
            sys::JVMTI_ERROR_UNSUPPORTED_VERSION => JvmtiError::UnsupportedVersion,
            sys::JVMTI_ERROR_NAMES_DONT_MATCH => JvmtiError::NamesDontMatch,
            sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_MODIFIERS_CHANGED => JvmtiError::UnsupportedRedefinitionClassModifiersChanged,
            sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_MODIFIERS_CHANGED => JvmtiError::UnsupportedRedefinitionMethodModifiersChanged,
            sys::JVMTI_ERROR_UNMODIFIABLE_CLASS => JvmtiError::UnmodifiableClass,
            sys::JVMTI_ERROR_NOT_AVAILABLE => JvmtiError::NotAvaliable,
            sys::JVMTI_ERROR_MUST_POSSESS_CAPABILITY => JvmtiError::MustPosessCapability,
            sys::JVMTI_ERROR_NULL_POINTER => JvmtiError::NullPointer,
            sys::JVMTI_ERROR_ABSENT_INFORMATION => JvmtiError::AbsentInformation,
            sys::JVMTI_ERROR_INVALID_EVENT_TYPE => JvmtiError::InvalidEventType,
            sys::JVMTI_ERROR_ILLEGAL_ARGUMENT => JvmtiError::IllegalArgument,
            sys::JVMTI_ERROR_NATIVE_METHOD => JvmtiError::NativeMethod,
            sys::JVMTI_ERROR_CLASS_LOADER_UNSUPPORTED => JvmtiError::ClassLoaderUnsupported,
            sys::JVMTI_ERROR_OUT_OF_MEMORY => JvmtiError::OutOfMemory,
            sys::JVMTI_ERROR_ACCESS_DENIED => JvmtiError::AccessDenied,
            sys::JVMTI_ERROR_WRONG_PHASE => JvmtiError::WrongPhase,
            sys::JVMTI_ERROR_INTERNAL => JvmtiError::Internal,
            sys::JVMTI_ERROR_UNATTACHED_THREAD => JvmtiError::UnattachedThread,
            sys::JVMTI_ERROR_INVALID_ENVIRONMENT => JvmtiError::InvalidEnvironment,
            _ => JvmtiError::UnsupportedError(error),
        }
    }
}


impl From<JvmtiError> for sys::jvmtiError {
    fn from(error: JvmtiError) -> Self {
        match error {
            JvmtiError::InvalidThread => sys::JVMTI_ERROR_INVALID_THREAD,
            JvmtiError::InvalidThreadGroup => sys::JVMTI_ERROR_INVALID_THREAD_GROUP,
            JvmtiError::InvalidPriority => sys::JVMTI_ERROR_INVALID_PRIORITY,
            JvmtiError::ThreadNotSuspended => sys::JVMTI_ERROR_THREAD_NOT_SUSPENDED,
            JvmtiError::ThreadSuspended => sys::JVMTI_ERROR_THREAD_SUSPENDED,
            JvmtiError::ThreadNotAlive => sys::JVMTI_ERROR_THREAD_NOT_ALIVE,
            JvmtiError::InvalidObject => sys::JVMTI_ERROR_INVALID_OBJECT,
            JvmtiError::InvalidClass => sys::JVMTI_ERROR_INVALID_CLASS,
            JvmtiError::ClassNotPrepared => sys::JVMTI_ERROR_CLASS_NOT_PREPARED,
            JvmtiError::InvalidMethodId => sys::JVMTI_ERROR_INVALID_METHODID,
            JvmtiError::InvalidLocation => sys::JVMTI_ERROR_INVALID_LOCATION,
            JvmtiError::InvalidFieldId => sys::JVMTI_ERROR_INVALID_FIELDID,
            JvmtiError::NoMoreFrames => sys::JVMTI_ERROR_NO_MORE_FRAMES,
            JvmtiError::OpaqueFrame => sys::JVMTI_ERROR_OPAQUE_FRAME,
            JvmtiError::TypeMismatch => sys::JVMTI_ERROR_TYPE_MISMATCH,
            JvmtiError::InvalidSlot => sys::JVMTI_ERROR_INVALID_SLOT,
            JvmtiError::Duplicate => sys::JVMTI_ERROR_DUPLICATE,
            JvmtiError::NotFound => sys::JVMTI_ERROR_NOT_FOUND,
            JvmtiError::InvalidMonitor => sys::JVMTI_ERROR_INVALID_MONITOR,
            JvmtiError::NotMonitorOwner => sys::JVMTI_ERROR_NOT_MONITOR_OWNER,
            JvmtiError::Interrupt => sys::JVMTI_ERROR_INTERRUPT,
            JvmtiError::InvalidClassFormat => sys::JVMTI_ERROR_INVALID_CLASS_FORMAT,
            JvmtiError::CircularClassDefinition => sys::JVMTI_ERROR_CIRCULAR_CLASS_DEFINITION,
            JvmtiError::FailsVerification => sys::JVMTI_ERROR_FAILS_VERIFICATION,
            JvmtiError::UnsupportedRedefinitionMethodAdded => sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_ADDED,
            JvmtiError::UnsupportedRedefinitionSchemaChanged => sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED,
            JvmtiError::InvalidTypeState => sys::JVMTI_ERROR_INVALID_TYPESTATE,
            JvmtiError::UnsupportedRedefinitionHierarchyChanged => sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_HIERARCHY_CHANGED,
            JvmtiError::UnsupportedRedefinitionMethodDeleted => sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_DELETED,
            JvmtiError::UnsupportedVersion => sys::JVMTI_ERROR_UNSUPPORTED_VERSION,
            JvmtiError::NamesDontMatch => sys::JVMTI_ERROR_NAMES_DONT_MATCH,
            JvmtiError::UnsupportedRedefinitionClassModifiersChanged => sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_MODIFIERS_CHANGED,
            JvmtiError::UnsupportedRedefinitionMethodModifiersChanged => sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_MODIFIERS_CHANGED,
            JvmtiError::UnmodifiableClass => sys::JVMTI_ERROR_UNMODIFIABLE_CLASS,
            JvmtiError::NotAvaliable => sys::JVMTI_ERROR_NOT_AVAILABLE,
            JvmtiError::MustPosessCapability => sys::JVMTI_ERROR_MUST_POSSESS_CAPABILITY,
            JvmtiError::NullPointer => sys::JVMTI_ERROR_NULL_POINTER,
            JvmtiError::AbsentInformation => sys::JVMTI_ERROR_ABSENT_INFORMATION,
            JvmtiError::InvalidEventType => sys::JVMTI_ERROR_INVALID_EVENT_TYPE,
            JvmtiError::IllegalArgument => sys::JVMTI_ERROR_ILLEGAL_ARGUMENT,
            JvmtiError::NativeMethod => sys::JVMTI_ERROR_NATIVE_METHOD,
            JvmtiError::ClassLoaderUnsupported => sys::JVMTI_ERROR_CLASS_LOADER_UNSUPPORTED,
            JvmtiError::OutOfMemory => sys::JVMTI_ERROR_OUT_OF_MEMORY,
            JvmtiError::AccessDenied => sys::JVMTI_ERROR_ACCESS_DENIED,
            JvmtiError::WrongPhase => sys::JVMTI_ERROR_WRONG_PHASE,
            JvmtiError::Internal => sys::JVMTI_ERROR_INTERNAL,
            JvmtiError::UnattachedThread => sys::JVMTI_ERROR_UNATTACHED_THREAD,
            JvmtiError::InvalidEnvironment => sys::JVMTI_ERROR_INVALID_ENVIRONMENT,
            JvmtiError::EmptyArgument => sys::JVMTI_CONSTANT_UNSUPPORTED,
            JvmtiError::UnsupportedError(error) => error
        }
    }
}