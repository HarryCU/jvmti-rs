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

mod breakpoint;
mod breakpoint_instance;
mod breakpoint_static;
mod capability;
mod class;
mod class_jni;
mod class_loader_search;
mod event_management;
mod extension_mechanism;
mod field;
mod field_instance;
mod field_static;
mod force_early_return;
mod general;
mod heap;
mod heap_1_0;
mod heap_1_0_jni;
mod heap_jni;
mod jni_function_interception;
mod local_variable;
mod memory_management;
mod method;
mod method_instance;
mod method_jni;
mod method_static;
mod object;
mod raw_monitor;
mod stack_frame;
mod system_properties;
mod thread;
mod thread_group;
mod timers;
mod watched_field;

#[macro_export]
macro_rules! jvmti_catch {
    ($jvmti:expr, $name:tt, |$e:ident:$type:ty| $callback:block) => {
        debug!("looking up JVMTIEnv method {}", stringify!($name));
        let call = |$e:$type| $callback;
        match &$jvmti.$name() {
            Ok(val) => call(val),
            Err(e) => error!("call jvmti an exception occurs: {}", e)
        }
    };
    ($jvmti:expr, $name:tt, |$e:ident:$type:ty| $callback:block $(, $args:expr )* ) => {
        debug!("looking up JVMTIEnv method {}", stringify!($name));
        let call = |$e:$type| $callback;
        match &$jvmti.$name($($args),*,) {
            Ok(val) => call(val),
            Err(e) => error!("call jvmti an exception occurs: {}", e)
        }
    };
}
