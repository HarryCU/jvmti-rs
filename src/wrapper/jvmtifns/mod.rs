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
mod class;
mod field;
mod force_early_return;
mod heap;
mod heap_1_0;
mod local_variable;
mod method;
mod object;
mod thread_group;
mod watched_field;
mod stack_frame;
mod method_instance;
mod method_static;
mod field_instance;
mod field_static;
mod breakpoint_static;
mod breakpoint_instance;

pub use memory_management::*;
pub use thread::*;
pub use general::*;
pub use event_management::*;
pub use system_properties::*;
pub use class_loader_search::*;
pub use timers::*;
pub use capability::*;
pub use extension_mechanism::*;
pub use jni_function_interception::*;
pub use raw_monitor::*;
pub use breakpoint::*;
pub use breakpoint_instance::*;
pub use breakpoint_static::*;
pub use class::*;
pub use field::*;
pub use field_instance::*;
pub use field_static::*;
pub use force_early_return::*;
pub use heap::*;
pub use heap_1_0::*;
pub use local_variable::*;
pub use method::*;
pub use method_instance::*;
pub use method_static::*;
pub use object::*;
pub use thread_group::*;
pub use watched_field::*;
pub use stack_frame::*;

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