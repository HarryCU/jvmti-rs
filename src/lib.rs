extern crate jni;
extern crate jni_sys;
#[macro_use]
extern crate lazy_static;

use std::os::raw::c_char;

use log::{error, info};

pub use wrapper::*;

use crate::{event::JEventManager, runner};
pub use crate::event::jvmti_event_breakpoint_handler;
pub use crate::event::jvmti_event_class_file_load_hook_handler;
pub use crate::event::jvmti_event_class_load_handler;
pub use crate::event::jvmti_event_class_prepare_handler;
pub use crate::event::jvmti_event_compiled_method_load_handler;
pub use crate::event::jvmti_event_compiled_method_unload_handler;
pub use crate::event::jvmti_event_data_dump_request_handler;
pub use crate::event::jvmti_event_dynamic_code_generated_handler;
pub use crate::event::jvmti_event_exception_catch_handler;
pub use crate::event::jvmti_event_exception_handler;
pub use crate::event::jvmti_event_field_access_handler;
pub use crate::event::jvmti_event_field_modification_handler;
pub use crate::event::jvmti_event_frame_pop_handler;
pub use crate::event::jvmti_event_garbage_collection_finish_handler;
pub use crate::event::jvmti_event_garbage_collection_start_handler;
pub use crate::event::jvmti_event_method_entry_handler;
pub use crate::event::jvmti_event_method_exit_handler;
pub use crate::event::jvmti_event_monitor_contended_enter_handler;
pub use crate::event::jvmti_event_monitor_contended_entered_handler;
pub use crate::event::jvmti_event_monitor_wait_handler;
pub use crate::event::jvmti_event_monitor_waited_handler;
pub use crate::event::jvmti_event_native_method_bind_handler;
pub use crate::event::jvmti_event_object_free_handler;
pub use crate::event::jvmti_event_resource_exhausted_handler;
pub use crate::event::jvmti_event_single_step_handler;
pub use crate::event::jvmti_event_thread_end_handler;
pub use crate::event::jvmti_event_thread_start_handler;
pub use crate::event::jvmti_event_vm_death_handler;
pub use crate::event::jvmti_event_vm_init_handler;
pub use crate::event::jvmti_event_vm_object_alloc_handler;
pub use crate::event::jvmti_event_vm_start_handler;

pub mod sys;
pub mod perf;

mod wrapper {
    pub use decoder::*;
    pub use descriptors::*;
    pub use enums::*;
    pub use facade::*;
    pub use jvmtienv::*;
    pub use jvmtifns::*;
    pub use metadata::*;
    pub use transforms::*;
    pub use utils::*;
    pub use vm::*;

    #[macro_use]
    mod macros;
    mod enums;
    mod descriptors;
    mod transforms;
    mod utils;

    mod decoder;
    mod metadata;
    mod jvmtifns;
    mod jvmtienv;
    mod vm;
    mod facade;

    pub mod errors;
    pub mod objects;
    pub mod runner;
    pub mod builder;
    pub mod event;
}

pub fn agent_on_load(vm: &JavaVM, options: *const c_char, initialize: fn(*const c_char, &mut JEventManager)) -> i32 {
    info!("Agent starting...");

    match runner::do_on_load(vm, options, initialize) {
        Ok(_) => 0,
        Err(e) => {
            error!("Failed to load agent: {}", e);
            return -1;
        }
    }
}

pub fn agent_on_unload(_vm: &JavaVM) {
    info!("Agent unloaded");
}