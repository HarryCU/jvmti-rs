
macro_rules! event_call {
    ($handler:expr, $name:tt $(, $args:expr )* ) => {{
        let result = panic::catch_unwind(|| {
            match EventAdjuster::global().$name {
                Some(func) => {
                    func($($args),*)
                }
                _ => return // debug!("Not Found handle '{}' event 11.", $handler)
            }
        });
        match result {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to handle '{}' event: {:?}", $handler, e);
            }
        }
    }};
}

macro_rules! empty_event_handlers {
    () =>{{
        EventHandlers {
            breakpoint: None,
            class_file_load_hook: None,
            class_load: None,
            class_prepare: None,
            compiled_method_load: None,
            compiled_method_unload: None,
            data_dump_request: None,
            dynamic_code_generated: None,
            exception: None,
            exception_catch: None,
            field_access: None,
            field_modification: None,
            frame_pop: None,
            garbage_collection_finish: None,
            garbage_collection_start: None,
            method_entry: None,
            method_exit: None,
            monitor_contended_enter: None,
            monitor_contended_entered: None,
            monitor_wait: None,
            monitor_waited: None,
            native_method_bind: None,
            object_free: None,
            resource_exhausted: None,
            single_step: None,
            thread_end: None,
            thread_start: None,
            vm_death: None,
            vm_init: None,
            vm_object_alloc: None,
            vm_start: None,
        }
    }}
}