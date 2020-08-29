pub use breakpoint::*;
pub use breakpoint_instance::*;
pub use breakpoint_static::*;
pub use capability::*;
pub use class::*;
pub use class_jni::*;
pub use class_loader_search::*;
pub use event_management::*;
pub use extension_mechanism::*;
pub use field::*;
pub use field_instance::*;
pub use field_static::*;
pub use force_early_return::*;
pub use general::*;
pub use heap::*;
pub use heap_1_0::*;
pub use heap_1_0_jni::*;
pub use heap_jni::*;
pub use jni_function_interception::*;
pub use local_variable::*;
pub use memory_management::*;
pub use method::*;
pub use method_instance::*;
pub use method_jni::*;
pub use method_static::*;
pub use object::*;
pub use raw_monitor::*;
pub use stack_frame::*;
pub use system_properties::*;
pub use thread::*;
pub use thread_group::*;
pub use timers::*;
pub use watched_field::*;

mod memory_management;
mod thread;
mod general;
mod event_management;
mod system_properties;
mod class_loader_search;
mod timers;
mod capability;
mod extension_mechanism;
mod jni_function_interception;
mod raw_monitor;
mod breakpoint;
mod breakpoint_static;
mod breakpoint_instance;
mod class;
mod class_jni;
mod field;
mod field_instance;
mod field_static;
mod force_early_return;
mod heap;
mod heap_jni;
mod heap_1_0;
mod local_variable;
mod method;
mod method_jni;
mod method_instance;
mod method_static;
mod object;
mod thread_group;
mod watched_field;
mod stack_frame;
mod heap_1_0_jni;

#[macro_export]
macro_rules! jvmti_catch {
    ($event:expr, $name:tt, $callback:block) => {
        debug!("looking up JVMTIEnv method {}", stringify!($name));
        let call = $callback;
        match &$event.jvmti.$name() {
            Ok(val) => call(val),
            Err(e) => error!("call jvmti an exception occurs: {}", e)
        }
    };
    ($event:expr, $name:tt, $callback:block $(, $args:expr )* ) => {
        debug!("looking up JVMTIEnv method {}", stringify!($name));
        let call = $callback;
        match &$event.jvmti.$name($($args),*,) {
            Ok(val) => call(val),
            Err(e) => error!("call jvmti an exception occurs: {}", e)
        }
    };
}