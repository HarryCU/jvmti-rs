#![allow(non_snake_case, non_camel_case_types)]

// @formatter:off

pub const JVMTI_CONSTANT_UNSUPPORTED                                     : u32 = 0;

pub type jvmtiThreadState = u32;
/* Thread State Flags */
pub const JVMTI_THREAD_STATE_ALIVE                                       : jvmtiThreadState = 0x0001;
pub const JVMTI_THREAD_STATE_TERMINATED                                  : jvmtiThreadState = 0x0002;
pub const JVMTI_THREAD_STATE_RUNNABLE                                    : jvmtiThreadState = 0x0004;
pub const JVMTI_THREAD_STATE_BLOCKED_ON_MONITOR_ENTER                    : jvmtiThreadState = 0x0400;
pub const JVMTI_THREAD_STATE_WAITING                                     : jvmtiThreadState = 0x0080;
pub const JVMTI_THREAD_STATE_WAITING_INDEFINITELY                        : jvmtiThreadState = 0x0010;
pub const JVMTI_THREAD_STATE_WAITING_WITH_TIMEOUT                        : jvmtiThreadState = 0x0020;
pub const JVMTI_THREAD_STATE_SLEEPING                                    : jvmtiThreadState = 0x0040;
pub const JVMTI_THREAD_STATE_IN_OBJECT_WAIT                              : jvmtiThreadState = 0x0100;
pub const JVMTI_THREAD_STATE_PARKED                                      : jvmtiThreadState = 0x0200;
pub const JVMTI_THREAD_STATE_SUSPENDED                                   : jvmtiThreadState = 0x100000;
pub const JVMTI_THREAD_STATE_INTERRUPTED                                 : jvmtiThreadState = 0x200000;
pub const JVMTI_THREAD_STATE_IN_NATIVE                                   : jvmtiThreadState = 0x400000;
pub const JVMTI_THREAD_STATE_VENDOR_1                                    : jvmtiThreadState = 0x10000000;
pub const JVMTI_THREAD_STATE_VENDOR_2                                    : jvmtiThreadState = 0x20000000;
pub const JVMTI_THREAD_STATE_VENDOR_3                                    : jvmtiThreadState = 0x40000000;


pub type jvmtiJavaLangThreadState = u32;
/* java.lang.Thread.State Conversion Masks */
pub const JVMTI_JAVA_LANG_THREAD_STATE_MASK                              : jvmtiJavaLangThreadState = JVMTI_THREAD_STATE_TERMINATED | JVMTI_THREAD_STATE_ALIVE | JVMTI_THREAD_STATE_RUNNABLE
                                                                                                     | JVMTI_THREAD_STATE_BLOCKED_ON_MONITOR_ENTER | JVMTI_THREAD_STATE_WAITING
                                                                                                     | JVMTI_THREAD_STATE_WAITING_INDEFINITELY | JVMTI_THREAD_STATE_WAITING_WITH_TIMEOUT;
pub const JVMTI_JAVA_LANG_THREAD_STATE_NEW                               : jvmtiJavaLangThreadState = 0;
pub const JVMTI_JAVA_LANG_THREAD_STATE_TERMINATED                        : jvmtiJavaLangThreadState = JVMTI_THREAD_STATE_TERMINATED;
pub const JVMTI_JAVA_LANG_THREAD_STATE_RUNNABLE                          : jvmtiJavaLangThreadState = JVMTI_THREAD_STATE_ALIVE | JVMTI_THREAD_STATE_RUNNABLE;
pub const JVMTI_JAVA_LANG_THREAD_STATE_BLOCKED                           : jvmtiJavaLangThreadState = JVMTI_THREAD_STATE_ALIVE | JVMTI_THREAD_STATE_BLOCKED_ON_MONITOR_ENTER;
pub const JVMTI_JAVA_LANG_THREAD_STATE_WAITING                           : jvmtiJavaLangThreadState = JVMTI_THREAD_STATE_ALIVE | JVMTI_THREAD_STATE_WAITING | JVMTI_THREAD_STATE_WAITING_INDEFINITELY;
pub const JVMTI_JAVA_LANG_THREAD_STATE_TIMED_WAITING                     : jvmtiJavaLangThreadState = JVMTI_THREAD_STATE_ALIVE | JVMTI_THREAD_STATE_WAITING | JVMTI_THREAD_STATE_WAITING_WITH_TIMEOUT;

pub type jvmtiThreadPriority = i32;
/* Thread Priority Constants */
pub const JVMTI_THREAD_MIN_PRIORITY                                      : jvmtiThreadPriority = 1;
pub const JVMTI_THREAD_NORM_PRIORITY                                     : jvmtiThreadPriority = 5;
pub const JVMTI_THREAD_MAX_PRIORITY                                      : jvmtiThreadPriority = 10;

pub type jvmtiHeapFilter = u32;
/* Heap Filter Flags */
pub const JVMTI_HEAP_FILTER_TAGGED                                       : jvmtiHeapFilter = 0x4;
pub const JVMTI_HEAP_FILTER_UNTAGGED                                     : jvmtiHeapFilter = 0x8;
pub const JVMTI_HEAP_FILTER_CLASS_TAGGED                                 : jvmtiHeapFilter = 0x10;
pub const JVMTI_HEAP_FILTER_CLASS_UNTAGGED                               : jvmtiHeapFilter = 0x20;

pub type jvmtiVisit = u32;
/* Heap Visit Control Flags */
pub const JVMTI_VISIT_OBJECTS                                            : jvmtiVisit = 0x100;
pub const JVMTI_VISIT_ABORT                                              : jvmtiVisit = 0x8000;

pub type jvmtiHeapReferenceKind = u32;
/* Heap Reference Enumeration */
pub const JVMTI_HEAP_REFERENCE_CLASS                                     : jvmtiHeapReferenceKind = 1;
pub const JVMTI_HEAP_REFERENCE_FIELD                                     : jvmtiHeapReferenceKind = 2;
pub const JVMTI_HEAP_REFERENCE_ARRAY_ELEMENT                             : jvmtiHeapReferenceKind = 3;
pub const JVMTI_HEAP_REFERENCE_CLASS_LOADER                              : jvmtiHeapReferenceKind = 4;
pub const JVMTI_HEAP_REFERENCE_SIGNERS                                   : jvmtiHeapReferenceKind = 5;
pub const JVMTI_HEAP_REFERENCE_PROTECTION_DOMAIN                         : jvmtiHeapReferenceKind = 6;
pub const JVMTI_HEAP_REFERENCE_INTERFACE                                 : jvmtiHeapReferenceKind = 7;
pub const JVMTI_HEAP_REFERENCE_STATIC_FIELD                              : jvmtiHeapReferenceKind = 8;
pub const JVMTI_HEAP_REFERENCE_CONSTANT_POOL                             : jvmtiHeapReferenceKind = 9;
pub const JVMTI_HEAP_REFERENCE_SUPERCLASS                                : jvmtiHeapReferenceKind = 10;
pub const JVMTI_HEAP_REFERENCE_JNI_GLOBAL                                : jvmtiHeapReferenceKind = 21;
pub const JVMTI_HEAP_REFERENCE_SYSTEM_CLASS                              : jvmtiHeapReferenceKind = 22;
pub const JVMTI_HEAP_REFERENCE_MONITOR                                   : jvmtiHeapReferenceKind = 23;
pub const JVMTI_HEAP_REFERENCE_STACK_LOCAL                               : jvmtiHeapReferenceKind = 24;
pub const JVMTI_HEAP_REFERENCE_JNI_LOCAL                                 : jvmtiHeapReferenceKind = 25;
pub const JVMTI_HEAP_REFERENCE_THREAD                                    : jvmtiHeapReferenceKind = 26;
pub const JVMTI_HEAP_REFERENCE_OTHER                                     : jvmtiHeapReferenceKind = 27;

pub type jvmtiPrimitiveType = u32;
/* Primitive Type Enumeration */
pub const JVMTI_PRIMITIVE_TYPE_BOOLEAN                                   : jvmtiPrimitiveType = 90;
pub const JVMTI_PRIMITIVE_TYPE_BYTE                                      : jvmtiPrimitiveType = 66;
pub const JVMTI_PRIMITIVE_TYPE_CHAR                                      : jvmtiPrimitiveType = 67;
pub const JVMTI_PRIMITIVE_TYPE_SHORT                                     : jvmtiPrimitiveType = 83;
pub const JVMTI_PRIMITIVE_TYPE_INT                                       : jvmtiPrimitiveType = 73;
pub const JVMTI_PRIMITIVE_TYPE_LONG                                      : jvmtiPrimitiveType = 74;
pub const JVMTI_PRIMITIVE_TYPE_FLOAT                                     : jvmtiPrimitiveType = 70;
pub const JVMTI_PRIMITIVE_TYPE_DOUBLE                                    : jvmtiPrimitiveType = 68;

pub type jvmtiHeapObjectFilter = u32;
/* Heap Object Filter Enumeration */
pub const JVMTI_HEAP_OBJECT_TAGGED                                       : jvmtiHeapObjectFilter = 1;
pub const JVMTI_HEAP_OBJECT_UNTAGGED                                     : jvmtiHeapObjectFilter = 2;
pub const JVMTI_HEAP_OBJECT_EITHER                                       : jvmtiHeapObjectFilter = 3;

pub type jvmtiHeapRootKind = u32;
/* Heap Root Kind Enumeration */
pub const JVMTI_HEAP_ROOT_JNI_GLOBAL                                     : jvmtiHeapRootKind = 1;
pub const JVMTI_HEAP_ROOT_SYSTEM_CLASS                                   : jvmtiHeapRootKind = 2;
pub const JVMTI_HEAP_ROOT_MONITOR                                        : jvmtiHeapRootKind = 3;
pub const JVMTI_HEAP_ROOT_STACK_LOCAL                                    : jvmtiHeapRootKind = 4;
pub const JVMTI_HEAP_ROOT_JNI_LOCAL                                      : jvmtiHeapRootKind = 5;
pub const JVMTI_HEAP_ROOT_THREAD                                         : jvmtiHeapRootKind = 6;
pub const JVMTI_HEAP_ROOT_OTHER                                          : jvmtiHeapRootKind = 7;

pub type jvmtiObjectReferenceKind = u32;
/* Object Reference Enumeration */
pub const JVMTI_REFERENCE_CLASS                                          : jvmtiObjectReferenceKind = 1;
pub const JVMTI_REFERENCE_FIELD                                          : jvmtiObjectReferenceKind = 2;
pub const JVMTI_REFERENCE_ARRAY_ELEMENT                                  : jvmtiObjectReferenceKind = 3;
pub const JVMTI_REFERENCE_CLASS_LOADER                                   : jvmtiObjectReferenceKind = 4;
pub const JVMTI_REFERENCE_SIGNERS                                        : jvmtiObjectReferenceKind = 5;
pub const JVMTI_REFERENCE_PROTECTION_DOMAIN                              : jvmtiObjectReferenceKind = 6;
pub const JVMTI_REFERENCE_INTERFACE                                      : jvmtiObjectReferenceKind = 7;
pub const JVMTI_REFERENCE_STATIC_FIELD                                   : jvmtiObjectReferenceKind = 8;
pub const JVMTI_REFERENCE_CONSTANT_POOL                                  : jvmtiObjectReferenceKind = 9;

pub type jvmtiIterationControl = u32;
/* Iteration Control Enumeration */
pub const JVMTI_ITERATION_CONTINUE                                       : jvmtiIterationControl = 1;
pub const JVMTI_ITERATION_IGNORE                                         : jvmtiIterationControl = 2;
pub const JVMTI_ITERATION_ABORT                                          : jvmtiIterationControl = 0;

pub type jvmtiClassStatus = u32;
/* Class Status Flags */
pub const JVMTI_CLASS_STATUS_VERIFIED                                    : jvmtiClassStatus = 1;
pub const JVMTI_CLASS_STATUS_PREPARED                                    : jvmtiClassStatus = 2;
pub const JVMTI_CLASS_STATUS_INITIALIZED                                 : jvmtiClassStatus = 4;
pub const JVMTI_CLASS_STATUS_ERROR                                       : jvmtiClassStatus = 8;
pub const JVMTI_CLASS_STATUS_ARRAY                                       : jvmtiClassStatus = 16;
pub const JVMTI_CLASS_STATUS_PRIMITIVE                                   : jvmtiClassStatus = 32;

pub type jvmtiEventMode = u32;
/* Event Enable/Disable */
pub const JVMTI_ENABLE                                                   : jvmtiEventMode = 1;
pub const JVMTI_DISABLE                                                  : jvmtiEventMode = 0;


pub type jvmtiParamTypes = u32;
/* Extension Function/Event Parameter Types */
pub const JVMTI_TYPE_JBYTE                                               : jvmtiParamTypes = 101;
pub const JVMTI_TYPE_JCHAR                                               : jvmtiParamTypes = 102;
pub const JVMTI_TYPE_JSHORT                                              : jvmtiParamTypes = 103;
pub const JVMTI_TYPE_JINT                                                : jvmtiParamTypes = 104;
pub const JVMTI_TYPE_JLONG                                               : jvmtiParamTypes = 105;
pub const JVMTI_TYPE_JFLOAT                                              : jvmtiParamTypes = 106;
pub const JVMTI_TYPE_JDOUBLE                                             : jvmtiParamTypes = 107;
pub const JVMTI_TYPE_JBOOLEAN                                            : jvmtiParamTypes = 108;
pub const JVMTI_TYPE_JOBJECT                                             : jvmtiParamTypes = 109;
pub const JVMTI_TYPE_JTHREAD                                             : jvmtiParamTypes = 110;
pub const JVMTI_TYPE_JCLASS                                              : jvmtiParamTypes = 111;
pub const JVMTI_TYPE_JVALUE                                              : jvmtiParamTypes = 112;
pub const JVMTI_TYPE_JFIELDID                                            : jvmtiParamTypes = 113;
pub const JVMTI_TYPE_JMETHODID                                           : jvmtiParamTypes = 114;
pub const JVMTI_TYPE_CCHAR                                               : jvmtiParamTypes = 115;
pub const JVMTI_TYPE_CVOID                                               : jvmtiParamTypes = 116;
pub const JVMTI_TYPE_JNIENV                                              : jvmtiParamTypes = 117;

pub type jvmtiParamKind = u32;
/* Extension Function/Event Parameter Kinds */
pub const JVMTI_KIND_IN                                                  : jvmtiParamKind = 91;
pub const JVMTI_KIND_IN_PTR                                              : jvmtiParamKind = 92;
pub const JVMTI_KIND_IN_BUF                                              : jvmtiParamKind = 93;
pub const JVMTI_KIND_ALLOC_BUF                                           : jvmtiParamKind = 94;
pub const JVMTI_KIND_ALLOC_ALLOC_BUF                                     : jvmtiParamKind = 95;
pub const JVMTI_KIND_OUT                                                 : jvmtiParamKind = 96;
pub const JVMTI_KIND_OUT_BUF                                             : jvmtiParamKind = 97;

pub type jvmtiTimerKind = u32;
/* Timer Kinds */
pub const JVMTI_TIMER_USER_CPU                                           : jvmtiTimerKind = 30;
pub const JVMTI_TIMER_TOTAL_CPU                                          : jvmtiTimerKind = 31;
pub const JVMTI_TIMER_ELAPSED                                            : jvmtiTimerKind = 32;

pub type jvmtiPhase = u32;
/* Phases of execution */
pub const JVMTI_PHASE_ONLOAD                                             : jvmtiPhase = 1;
pub const JVMTI_PHASE_PRIMORDIAL                                         : jvmtiPhase = 2;
pub const JVMTI_PHASE_START                                              : jvmtiPhase = 6;
pub const JVMTI_PHASE_LIVE                                               : jvmtiPhase = 4;
pub const JVMTI_PHASE_DEAD                                               : jvmtiPhase = 8;

pub type jvmtiVersionInterface = u32;
/* Version Interface Types */
pub const JVMTI_VERSION_INTERFACE_JNI                                    : jvmtiVersionInterface = 0x00000000;
pub const JVMTI_VERSION_INTERFACE_JVMTI                                  : jvmtiVersionInterface = 0x30000000;

pub type jvmtiVersionMask = u32;
/* Version Masks */
pub const JVMTI_VERSION_MASK_INTERFACE_TYPE                              : jvmtiVersionMask = 0x70000000;
pub const JVMTI_VERSION_MASK_MAJOR                                       : jvmtiVersionMask = 0x0FFF0000;
pub const JVMTI_VERSION_MASK_MINOR                                       : jvmtiVersionMask = 0x0000FF00;
pub const JVMTI_VERSION_MASK_MICRO                                       : jvmtiVersionMask = 0x000000FF;

pub type jvmtiVersionShift = u32;
/* Version Shifts */
pub const JVMTI_VERSION_SHIFT_MAJOR                                      : jvmtiVersionShift = 16;
pub const JVMTI_VERSION_SHIFT_MINOR                                      : jvmtiVersionShift = 8;
pub const JVMTI_VERSION_SHIFT_MICRO                                      : jvmtiVersionShift = 0;

pub type jvmtiVerboseFlag = u32;
/* Verbose Flag Enumeration */
pub const JVMTI_VERBOSE_OTHER                                            : jvmtiVerboseFlag = 0;
pub const JVMTI_VERBOSE_GC                                               : jvmtiVerboseFlag = 1;
pub const JVMTI_VERBOSE_CLASS                                            : jvmtiVerboseFlag = 2;
pub const JVMTI_VERBOSE_JNI                                              : jvmtiVerboseFlag = 4;

pub type jvmtiJlocationFormat = u32;
/* jlocation Format Enumeration */
pub const JVMTI_JLOCATION_JVMBCI                                         : jvmtiJlocationFormat = 1;
pub const JVMTI_JLOCATION_MACHINEPC                                      : jvmtiJlocationFormat = 2;
pub const JVMTI_JLOCATION_OTHER                                          : jvmtiJlocationFormat = 0;

pub type jvmtiResourceExhausted = u32;
/* Resource Exhaustion Flags */
pub const JVMTI_RESOURCE_EXHAUSTED_OOM_ERROR                             : jvmtiResourceExhausted = 0x0001;
pub const JVMTI_RESOURCE_EXHAUSTED_JAVA_HEAP                             : jvmtiResourceExhausted = 0x0002;
pub const JVMTI_RESOURCE_EXHAUSTED_THREADS                               : jvmtiResourceExhausted = 0x0004;

pub type jvmtiError = u32;
/* Errors */
pub const JVMTI_ERROR_NONE                                               : jvmtiError = 0;
pub const JVMTI_ERROR_INVALID_THREAD                                     : jvmtiError = 10;
pub const JVMTI_ERROR_INVALID_THREAD_GROUP                               : jvmtiError = 11;
pub const JVMTI_ERROR_INVALID_PRIORITY                                   : jvmtiError = 12;
pub const JVMTI_ERROR_THREAD_NOT_SUSPENDED                               : jvmtiError = 13;
pub const JVMTI_ERROR_THREAD_SUSPENDED                                   : jvmtiError = 14;
pub const JVMTI_ERROR_THREAD_NOT_ALIVE                                   : jvmtiError = 15;
pub const JVMTI_ERROR_INVALID_OBJECT                                     : jvmtiError = 20;
pub const JVMTI_ERROR_INVALID_CLASS                                      : jvmtiError = 21;
pub const JVMTI_ERROR_CLASS_NOT_PREPARED                                 : jvmtiError = 22;
pub const JVMTI_ERROR_INVALID_METHODID                                   : jvmtiError = 23;
pub const JVMTI_ERROR_INVALID_LOCATION                                   : jvmtiError = 24;
pub const JVMTI_ERROR_INVALID_FIELDID                                    : jvmtiError = 25;
pub const JVMTI_ERROR_NO_MORE_FRAMES                                     : jvmtiError = 31;
pub const JVMTI_ERROR_OPAQUE_FRAME                                       : jvmtiError = 32;
pub const JVMTI_ERROR_TYPE_MISMATCH                                      : jvmtiError = 34;
pub const JVMTI_ERROR_INVALID_SLOT                                       : jvmtiError = 35;
pub const JVMTI_ERROR_DUPLICATE                                          : jvmtiError = 40;
pub const JVMTI_ERROR_NOT_FOUND                                          : jvmtiError = 41;
pub const JVMTI_ERROR_INVALID_MONITOR                                    : jvmtiError = 50;
pub const JVMTI_ERROR_NOT_MONITOR_OWNER                                  : jvmtiError = 51;
pub const JVMTI_ERROR_INTERRUPT                                          : jvmtiError = 52;
pub const JVMTI_ERROR_INVALID_CLASS_FORMAT                               : jvmtiError = 60;
pub const JVMTI_ERROR_CIRCULAR_CLASS_DEFINITION                          : jvmtiError = 61;
pub const JVMTI_ERROR_FAILS_VERIFICATION                                 : jvmtiError = 62;
pub const JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_ADDED              : jvmtiError = 63;
pub const JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED            : jvmtiError = 64;
pub const JVMTI_ERROR_INVALID_TYPESTATE                                  : jvmtiError = 65;
pub const JVMTI_ERROR_UNSUPPORTED_REDEFINITION_HIERARCHY_CHANGED         : jvmtiError = 66;
pub const JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_DELETED            : jvmtiError = 67;
pub const JVMTI_ERROR_UNSUPPORTED_VERSION                                : jvmtiError = 68;
pub const JVMTI_ERROR_NAMES_DONT_MATCH                                   : jvmtiError = 69;
pub const JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_MODIFIERS_CHANGED   : jvmtiError = 70;
pub const JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_MODIFIERS_CHANGED  : jvmtiError = 71;
pub const JVMTI_ERROR_UNMODIFIABLE_CLASS                                 : jvmtiError = 79;
pub const JVMTI_ERROR_NOT_AVAILABLE                                      : jvmtiError = 98;
pub const JVMTI_ERROR_MUST_POSSESS_CAPABILITY                            : jvmtiError = 99;
pub const JVMTI_ERROR_NULL_POINTER                                       : jvmtiError = 100;
pub const JVMTI_ERROR_ABSENT_INFORMATION                                 : jvmtiError = 101;
pub const JVMTI_ERROR_INVALID_EVENT_TYPE                                 : jvmtiError = 102;
pub const JVMTI_ERROR_ILLEGAL_ARGUMENT                                   : jvmtiError = 103;
pub const JVMTI_ERROR_NATIVE_METHOD                                      : jvmtiError = 104;
pub const JVMTI_ERROR_CLASS_LOADER_UNSUPPORTED                           : jvmtiError = 106;
pub const JVMTI_ERROR_OUT_OF_MEMORY                                      : jvmtiError = 110;
pub const JVMTI_ERROR_ACCESS_DENIED                                      : jvmtiError = 111;
pub const JVMTI_ERROR_WRONG_PHASE                                        : jvmtiError = 112;
pub const JVMTI_ERROR_INTERNAL                                           : jvmtiError = 113;
pub const JVMTI_ERROR_UNATTACHED_THREAD                                  : jvmtiError = 115;
pub const JVMTI_ERROR_INVALID_ENVIRONMENT                                : jvmtiError = 116;
pub const JVMTI_ERROR_MAX                                                : jvmtiError = 116;

pub type jvmtiEvent = u32;
/* Event IDs */
pub const JVMTI_MIN_EVENT_TYPE_VAL                                       : jvmtiEvent = 50;
pub const JVMTI_EVENT_VM_INIT                                            : jvmtiEvent = 50;
pub const JVMTI_EVENT_VM_DEATH                                           : jvmtiEvent = 51;
pub const JVMTI_EVENT_THREAD_START                                       : jvmtiEvent = 52;
pub const JVMTI_EVENT_THREAD_END                                         : jvmtiEvent = 53;
pub const JVMTI_EVENT_CLASS_FILE_LOAD_HOOK                               : jvmtiEvent = 54;
pub const JVMTI_EVENT_CLASS_LOAD                                         : jvmtiEvent = 55;
pub const JVMTI_EVENT_CLASS_PREPARE                                      : jvmtiEvent = 56;
pub const JVMTI_EVENT_VM_START                                           : jvmtiEvent = 57;
pub const JVMTI_EVENT_EXCEPTION                                          : jvmtiEvent = 58;
pub const JVMTI_EVENT_EXCEPTION_CATCH                                    : jvmtiEvent = 59;
pub const JVMTI_EVENT_SINGLE_STEP                                        : jvmtiEvent = 60;
pub const JVMTI_EVENT_FRAME_POP                                          : jvmtiEvent = 61;
pub const JVMTI_EVENT_BREAKPOINT                                         : jvmtiEvent = 62;
pub const JVMTI_EVENT_FIELD_ACCESS                                       : jvmtiEvent = 63;
pub const JVMTI_EVENT_FIELD_MODIFICATION                                 : jvmtiEvent = 64;
pub const JVMTI_EVENT_METHOD_ENTRY                                       : jvmtiEvent = 65;
pub const JVMTI_EVENT_METHOD_EXIT                                        : jvmtiEvent = 66;
pub const JVMTI_EVENT_NATIVE_METHOD_BIND                                 : jvmtiEvent = 67;
pub const JVMTI_EVENT_COMPILED_METHOD_LOAD                               : jvmtiEvent = 68;
pub const JVMTI_EVENT_COMPILED_METHOD_UNLOAD                             : jvmtiEvent = 69;
pub const JVMTI_EVENT_DYNAMIC_CODE_GENERATED                             : jvmtiEvent = 70;
pub const JVMTI_EVENT_DATA_DUMP_REQUEST                                  : jvmtiEvent = 71;
pub const JVMTI_EVENT_MONITOR_WAIT                                       : jvmtiEvent = 73;
pub const JVMTI_EVENT_MONITOR_WAITED                                     : jvmtiEvent = 74;
pub const JVMTI_EVENT_MONITOR_CONTENDED_ENTER                            : jvmtiEvent = 75;
pub const JVMTI_EVENT_MONITOR_CONTENDED_ENTERED                          : jvmtiEvent = 76;
pub const JVMTI_EVENT_RESOURCE_EXHAUSTED                                 : jvmtiEvent = 80;
pub const JVMTI_EVENT_GARBAGE_COLLECTION_START                           : jvmtiEvent = 81;
pub const JVMTI_EVENT_GARBAGE_COLLECTION_FINISH                          : jvmtiEvent = 82;
pub const JVMTI_EVENT_OBJECT_FREE                                        : jvmtiEvent = 83;
pub const JVMTI_EVENT_VM_OBJECT_ALLOC                                    : jvmtiEvent = 84;
pub const JVMTI_MAX_EVENT_TYPE_VAL                                       : jvmtiEvent = 84;

// @formatter:on