use crate::sys;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
pub struct JCapabilities<'a> {
    caps: sys::jvmtiCapabilities,
    lifetime: PhantomData<&'a ()>,
}

impl<'a> JCapabilities<'a> {
    pub fn new() -> JCapabilities<'a> {
        let caps = sys::jvmtiCapabilities {
            _bitfield_1: sys::jvmtiCapabilities::newEmptyBitfield()
        };
        JCapabilities {
            caps,
            lifetime: PhantomData,
        }
    }

    pub fn merge(other: sys::jvmtiCapabilities) -> JCapabilities<'a> {
        let mut item = JCapabilities::new();

        if other.can_tag_objects() == 1 { item.can_tag_objects() }
        if other.can_generate_field_modification_events() == 1 { item.can_generate_field_modification_events() }
        if other.can_generate_field_access_events() == 1 { item.can_generate_field_access_events() }
        if other.can_get_bytecodes() == 1 { item.can_get_bytecodes() }
        if other.can_get_synthetic_attribute() == 1 { item.can_get_synthetic_attribute() }
        if other.can_get_owned_monitor_info() == 1 { item.can_get_owned_monitor_info() }
        if other.can_get_current_contended_monitor() == 1 { item.can_get_current_contended_monitor() }
        if other.can_get_monitor_info() == 1 { item.can_get_monitor_info() }
        if other.can_pop_frame() == 1 { item.can_pop_frame() }
        if other.can_redefine_classes() == 1 { item.can_redefine_classes() }
        if other.can_signal_thread() == 1 { item.can_signal_thread() }
        if other.can_get_source_file_name() == 1 { item.can_get_source_file_name() }
        if other.can_get_line_numbers() == 1 { item.can_get_line_numbers() }
        if other.can_get_source_debug_extension() == 1 { item.can_get_source_debug_extension() }
        if other.can_access_local_variables() == 1 { item.can_access_local_variables() }
        if other.can_maintain_original_method_order() == 1 { item.can_maintain_original_method_order() }
        if other.can_generate_single_step_events() == 1 { item.can_generate_single_step_events() }
        if other.can_generate_exception_events() == 1 { item.can_generate_exception_events() }
        if other.can_generate_frame_pop_events() == 1 { item.can_generate_frame_pop_events() }
        if other.can_generate_breakpoint_events() == 1 { item.can_generate_breakpoint_events() }
        if other.can_suspend() == 1 { item.can_suspend() }
        if other.can_redefine_any_class() == 1 { item.can_redefine_any_class() }
        if other.can_get_current_thread_cpu_time() == 1 { item.can_get_current_thread_cpu_time() }
        if other.can_get_thread_cpu_time() == 1 { item.can_get_thread_cpu_time() }
        if other.can_generate_method_entry_events() == 1 { item.can_generate_method_entry_events() }
        if other.can_generate_method_exit_events() == 1 { item.can_generate_method_exit_events() }
        if other.can_generate_all_class_hook_events() == 1 { item.can_generate_all_class_hook_events() }
        if other.can_generate_compiled_method_load_events() == 1 { item.can_generate_compiled_method_load_events() }
        if other.can_generate_monitor_events() == 1 { item.can_generate_monitor_events() }
        if other.can_generate_vm_object_alloc_events() == 1 { item.can_generate_vm_object_alloc_events() }
        if other.can_generate_native_method_bind_events() == 1 { item.can_generate_native_method_bind_events() }
        if other.can_generate_garbage_collection_events() == 1 { item.can_generate_garbage_collection_events() }
        if other.can_generate_object_free_events() == 1 { item.can_generate_object_free_events() }
        if other.can_force_early_return() == 1 { item.can_force_early_return() }
        if other.can_get_owned_monitor_stack_depth_info() == 1 { item.can_get_owned_monitor_stack_depth_info() }
        if other.can_get_constant_pool() == 1 { item.can_get_constant_pool() }
        if other.can_set_native_method_prefix() == 1 { item.can_set_native_method_prefix() }
        if other.can_retransform_classes() == 1 { item.can_retransform_classes() }
        if other.can_retransform_any_class() == 1 { item.can_retransform_any_class() }
        if other.can_generate_resource_exhaustion_heap_events() == 1 { item.can_generate_resource_exhaustion_heap_events() }
        if other.can_generate_resource_exhaustion_heap_events() == 1 { item.can_generate_resource_exhaustion_threads_events() }

        item
    }

    pub fn can_tag_objects(&mut self) { self.caps.set_can_tag_objects(1) }
    pub fn can_generate_field_modification_events(&mut self) { self.caps.set_can_generate_field_modification_events(1) }
    pub fn can_generate_field_access_events(&mut self) { self.caps.set_can_generate_field_access_events(1) }
    pub fn can_get_bytecodes(&mut self) { self.caps.set_can_get_bytecodes(1) }
    pub fn can_get_synthetic_attribute(&mut self) { self.caps.set_can_get_synthetic_attribute(1) }
    pub fn can_get_owned_monitor_info(&mut self) { self.caps.set_can_get_owned_monitor_info(1) }
    pub fn can_get_current_contended_monitor(&mut self) { self.caps.set_can_get_current_contended_monitor(1) }
    pub fn can_get_monitor_info(&mut self) { self.caps.set_can_get_monitor_info(1) }
    pub fn can_pop_frame(&mut self) { self.caps.set_can_pop_frame(1) }
    pub fn can_redefine_classes(&mut self) { self.caps.set_can_redefine_classes(1) }
    pub fn can_signal_thread(&mut self) { self.caps.set_can_signal_thread(1) }
    pub fn can_get_source_file_name(&mut self) { self.caps.set_can_get_source_file_name(1) }
    pub fn can_get_line_numbers(&mut self) { self.caps.set_can_get_line_numbers(1) }
    pub fn can_get_source_debug_extension(&mut self) { self.caps.set_can_get_source_debug_extension(1) }
    pub fn can_access_local_variables(&mut self) { self.caps.set_can_access_local_variables(1) }
    pub fn can_maintain_original_method_order(&mut self) { self.caps.set_can_maintain_original_method_order(1) }
    pub fn can_generate_single_step_events(&mut self) { self.caps.set_can_generate_single_step_events(1) }
    pub fn can_generate_exception_events(&mut self) { self.caps.set_can_generate_exception_events(1) }
    pub fn can_generate_frame_pop_events(&mut self) { self.caps.set_can_generate_frame_pop_events(1) }
    pub fn can_generate_breakpoint_events(&mut self) { self.caps.set_can_generate_breakpoint_events(1) }
    pub fn can_suspend(&mut self) { self.caps.set_can_suspend(1) }
    pub fn can_redefine_any_class(&mut self) { self.caps.set_can_redefine_any_class(1) }
    pub fn can_get_current_thread_cpu_time(&mut self) { self.caps.set_can_get_current_thread_cpu_time(1) }
    pub fn can_get_thread_cpu_time(&mut self) { self.caps.set_can_get_thread_cpu_time(1) }
    pub fn can_generate_method_entry_events(&mut self) { self.caps.set_can_generate_method_entry_events(1) }
    pub fn can_generate_method_exit_events(&mut self) { self.caps.set_can_generate_method_exit_events(1) }
    pub fn can_generate_all_class_hook_events(&mut self) { self.caps.set_can_generate_all_class_hook_events(1) }
    pub fn can_generate_compiled_method_load_events(&mut self) { self.caps.set_can_generate_compiled_method_load_events(1) }
    pub fn can_generate_monitor_events(&mut self) { self.caps.set_can_generate_monitor_events(1) }
    pub fn can_generate_vm_object_alloc_events(&mut self) { self.caps.set_can_generate_vm_object_alloc_events(1) }
    pub fn can_generate_native_method_bind_events(&mut self) { self.caps.set_can_generate_native_method_bind_events(1) }
    pub fn can_generate_garbage_collection_events(&mut self) { self.caps.set_can_generate_garbage_collection_events(1) }
    pub fn can_generate_object_free_events(&mut self) { self.caps.set_can_generate_object_free_events(1) }
    pub fn can_force_early_return(&mut self) { self.caps.set_can_force_early_return(1) }
    pub fn can_get_owned_monitor_stack_depth_info(&mut self) { self.caps.set_can_get_owned_monitor_stack_depth_info(1) }
    pub fn can_get_constant_pool(&mut self) { self.caps.set_can_get_constant_pool(1) }
    pub fn can_set_native_method_prefix(&mut self) { self.caps.set_can_set_native_method_prefix(1) }
    pub fn can_retransform_classes(&mut self) { self.caps.set_can_retransform_classes(1) }
    pub fn can_retransform_any_class(&mut self) { self.caps.set_can_retransform_any_class(1) }
    pub fn can_generate_resource_exhaustion_heap_events(&mut self) { self.caps.set_can_generate_resource_exhaustion_heap_events(1) }
    pub fn can_generate_resource_exhaustion_threads_events(&mut self) { self.caps.set_can_generate_resource_exhaustion_threads_events(1) }
}

impl<'a> From<JCapabilities<'a>> for sys::jvmtiCapabilities {
    fn from(value: JCapabilities<'a>) -> Self {
        value.caps
    }
}