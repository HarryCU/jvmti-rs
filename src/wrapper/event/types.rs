use jni::JNIEnv;
use crate::sys::{JClass, JMethodID, jvalue, JFieldID, JObject, jint, jlong, jmemory};
use crate::wrapper::{JVMTIEnv, JThreadID, JLocation, JAddrLocationMap, JCompiledMethodLoadRecord};
use std::os::raw::{c_char, c_void, c_uchar};

pub type EventBreakpointFn = Option<fn(event: BreakpointEvent)>;

pub type EventClassFileLoadHookFn = Option<fn(event: ClassFileLoadHookEvent)>;

pub type EventClassLoadFn = Option<fn(event: ClassLoadEvent)>;

pub type EventClassPrepareFn = Option<fn(event: ClassPrepareEvent)>;

pub type CompiledMethodLoadFn = Option<fn(event: CompiledMethodLoadEvent)>;

pub type CompiledMethodUnloadFn = Option<fn(event: CompiledMethodUnloadEvent)>;

pub type DataDumpRequestFn = Option<fn(event: DataDumpRequestEvent)>;

pub type DynamicCodeGeneratedFn = Option<fn(event: DynamicCodeGeneratedEvent)>;

pub type ExceptionFn = Option<fn(event: ExceptionEvent)>;

pub type ExceptionCatchFn = Option<fn(event: ExceptionCatchEvent)>;

pub type FieldAccessFn = Option<fn(event: FieldAccessEvent)>;

pub type FieldModificationFn = Option<fn(event: FieldModificationEvent)>;

pub type FramePopFn = Option<fn(event: FramePopEvent)>;

pub type GarbageCollectionFinishFn = Option<fn(event: GarbageCollectionFinishEvent)>;

pub type GarbageCollectionStartFn = Option<fn(event: GarbageCollectionStartEvent)>;

pub type MethodEntryFn = Option<fn(event: MethodEntryEvent)>;

pub type MethodExitFn = Option<fn(event: MethodExitEvent)>;

pub type MonitorContendedEnterFn = Option<fn(event: MonitorContendedEnterEvent)>;

pub type MonitorContendedEnteredFn = Option<fn(event: MonitorContendedEnteredEvent)>;

pub type MonitorWaitFn = Option<fn(event: MonitorWaitEvent)>;

pub type MonitorWaitedFn = Option<fn(event: MonitorWaitedEvent)>;

pub type NativeMethodBindFn = Option<fn(event: NativeMethodBindEvent)>;

pub type ObjectFreeFn = Option<fn(event: ObjectFreeEvent)>;

pub type ResourceExhaustedFn = Option<fn(event: ResourceExhaustedEvent)>;

pub type SingleStepFn = Option<fn(event: SingleStepEvent)>;

pub type ThreadEndFn = Option<fn(event: ThreadEndEvent)>;

pub type ThreadStartFn = Option<fn(event: ThreadStartEvent)>;

pub type VmDeathFn = Option<fn(event: VmDeathEvent)>;

pub type VmInitFn = Option<fn(event: VmInitEvent)>;

pub type VmObjectAllocFn = Option<fn(event: VmObjectAllocEvent)>;

pub type VmStartFn = Option<fn(event: VmStartEvent)>;


pub struct BreakpointEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub jni: &'a JNIEnv<'a>,
    pub thread: JThreadID<'a>,
    pub method: JMethodID<'a>,
    pub location: JLocation<'a>,
}

pub struct ClassFileLoadHookEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub jni: &'a JNIEnv<'a>,
    pub class_being_redefined: JClass<'a>,
    pub loader: JObject<'a>,
    pub name: String,
    pub protection_domain: JObject<'a>,
    pub class_data_len: jint,
    pub class_data: *const c_uchar,
    pub new_class_data_len: *mut jint,
    pub new_class_data: *mut jmemory,
}

pub struct ClassLoadEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub jni: &'a JNIEnv<'a>,
    pub thread: JThreadID<'a>,
    pub klass: JClass<'a>,
}

pub struct ClassPrepareEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub jni: &'a JNIEnv<'a>,
    pub thread: JThreadID<'a>,
    pub klass: JClass<'a>,
}

pub struct CompiledMethodLoadEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub method: JMethodID<'a>,
    pub code_size: jint,
    pub code_addr: *const c_void,
    pub addr_location_map: Vec<JAddrLocationMap<'a>>,
    pub compiled_records: &'a Option<Vec<JCompiledMethodLoadRecord<'a>>>,
}

pub struct CompiledMethodUnloadEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub method: JMethodID<'a>,
    pub code_addr: *const c_void,
}

pub struct DataDumpRequestEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>
}

pub struct DynamicCodeGeneratedEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub name: String,
    pub address: *const c_void,
    pub length: jint,
}

pub struct ExceptionEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub jni: &'a JNIEnv<'a>,
    pub thread: JThreadID<'a>,
    pub method: JMethodID<'a>,
    pub location: JLocation<'a>,
    pub exception: JObject<'a>,
    pub catch_method: JMethodID<'a>,
    pub catch_location: JLocation<'a>,
}

pub struct ExceptionCatchEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub jni: &'a JNIEnv<'a>,
    pub thread: JThreadID<'a>,
    pub method: JMethodID<'a>,
    pub location: JLocation<'a>,
    pub exception: JObject<'a>,
}

pub struct FieldAccessEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub jni: &'a JNIEnv<'a>,
    pub thread: JThreadID<'a>,
    pub method: JMethodID<'a>,
    pub location: JLocation<'a>,
    pub field_klass: JClass<'a>,
    pub object: JObject<'a>,
    pub field: JFieldID<'a>,
}

pub struct FieldModificationEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub jni: &'a JNIEnv<'a>,
    pub thread: JThreadID<'a>,
    pub method: JMethodID<'a>,
    pub location: JLocation<'a>,
    pub field_klass: JClass<'a>,
    pub object: JObject<'a>,
    pub field: JFieldID<'a>,
    pub signature_type: c_char,
    pub new_value: jvalue,
}

pub struct FramePopEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub jni: &'a JNIEnv<'a>,
    pub thread: JThreadID<'a>,
    pub method: JMethodID<'a>,
    pub was_popped_by_exception: bool,
}

pub struct GarbageCollectionFinishEvent<'a> { pub jvmti: &'a JVMTIEnv<'a> }

pub struct GarbageCollectionStartEvent<'a> { pub jvmti: &'a JVMTIEnv<'a> }

pub struct MethodEntryEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub jni: &'a JNIEnv<'a>,
    pub thread: JThreadID<'a>,
    pub method: JMethodID<'a>,
}

pub struct MethodExitEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub jni: &'a JNIEnv<'a>,
    pub thread: JThreadID<'a>,
    pub method: JMethodID<'a>,
    pub was_popped_by_exception: bool,
    pub return_value: jvalue,
}

pub struct MonitorContendedEnterEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub jni: &'a JNIEnv<'a>,
    pub thread: JThreadID<'a>,
    pub object: JObject<'a>,
}

pub struct MonitorContendedEnteredEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub jni: &'a JNIEnv<'a>,
    pub thread: JThreadID<'a>,
    pub object: JObject<'a>,
}

pub struct MonitorWaitEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub jni: &'a JNIEnv<'a>,
    pub thread: JThreadID<'a>,
    pub object: JObject<'a>,
    pub timeout: jlong,
}

pub struct MonitorWaitedEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub jni: &'a JNIEnv<'a>,
    pub thread: JThreadID<'a>,
    pub object: JObject<'a>,
    pub timed_out: bool,
}

pub struct NativeMethodBindEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub jni: &'a JNIEnv<'a>,
    pub thread: JThreadID<'a>,
    pub method: JMethodID<'a>,
    pub address: *mut c_void,
    pub new_address_ptr: *mut *mut c_void,
}

pub struct ObjectFreeEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub tag: jlong,
}

pub struct ResourceExhaustedEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub jni: &'a JNIEnv<'a>,
    pub flags: jint,
    pub reserved: *const c_void,
    pub description: String,
}

pub struct SingleStepEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub jni: &'a JNIEnv<'a>,
    pub thread: JThreadID<'a>,
    pub method: JMethodID<'a>,
    pub location: JLocation<'a>,
}

pub struct ThreadEndEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub jni: &'a JNIEnv<'a>,
    pub thread: JThreadID<'a>,
}

pub struct ThreadStartEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub jni: &'a JNIEnv<'a>,
    pub thread: JThreadID<'a>,
}

pub struct VmDeathEvent<'a> { pub jvmti: &'a JVMTIEnv<'a>, pub jni: &'a JNIEnv<'a> }

pub struct VmInitEvent<'a> { pub jvmti: &'a JVMTIEnv<'a>, pub jni: &'a JNIEnv<'a>, pub thread: JThreadID<'a> }

pub struct VmObjectAllocEvent<'a> {
    pub jvmti: &'a JVMTIEnv<'a>,
    pub jni: &'a JNIEnv<'a>,
    pub thread: JThreadID<'a>,
    pub object: JObject<'a>,
    pub object_klass: JClass<'a>,
    pub size: jlong,
}

pub struct VmStartEvent<'a> { pub jvmti: &'a JVMTIEnv<'a>, pub jni: &'a JNIEnv<'a> }
