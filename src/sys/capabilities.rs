#![allow(non_snake_case, non_camel_case_types)]

use std::os::raw::c_uint;
use std::mem;
use crate::sys::r#type::jvmtiCapabilities;
use crate::sys::utils::BitfieldUnit;

impl jvmtiCapabilities {
    #[inline]
    pub fn can_tag_objects(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_tag_objects(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_generate_field_modification_events(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_generate_field_modification_events(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_generate_field_access_events(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_generate_field_access_events(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_get_bytecodes(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_get_bytecodes(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_get_synthetic_attribute(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_get_synthetic_attribute(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_get_owned_monitor_info(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_get_owned_monitor_info(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_get_current_contended_monitor(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(6usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_get_current_contended_monitor(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_get_monitor_info(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_get_monitor_info(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_pop_frame(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(8usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_pop_frame(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_redefine_classes(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(9usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_redefine_classes(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(9usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_signal_thread(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(10usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_signal_thread(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(10usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_get_source_file_name(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(11usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_get_source_file_name(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(11usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_get_line_numbers(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(12usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_get_line_numbers(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(12usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_get_source_debug_extension(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(13usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_get_source_debug_extension(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(13usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_access_local_variables(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(14usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_access_local_variables(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(14usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_maintain_original_method_order(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(15usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_maintain_original_method_order(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(15usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_generate_single_step_events(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(16usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_generate_single_step_events(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(16usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_generate_exception_events(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(17usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_generate_exception_events(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(17usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_generate_frame_pop_events(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(18usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_generate_frame_pop_events(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(18usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_generate_breakpoint_events(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(19usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_generate_breakpoint_events(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(19usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_suspend(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(20usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_suspend(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(20usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_redefine_any_class(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(21usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_redefine_any_class(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(21usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_get_current_thread_cpu_time(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(22usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_get_current_thread_cpu_time(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(22usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_get_thread_cpu_time(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(23usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_get_thread_cpu_time(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(23usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_generate_method_entry_events(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(24usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_generate_method_entry_events(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(24usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_generate_method_exit_events(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(25usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_generate_method_exit_events(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(25usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_generate_all_class_hook_events(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(26usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_generate_all_class_hook_events(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(26usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_generate_compiled_method_load_events(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(27usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_generate_compiled_method_load_events(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(27usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_generate_monitor_events(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(28usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_generate_monitor_events(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(28usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_generate_vm_object_alloc_events(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(29usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_generate_vm_object_alloc_events(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(29usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_generate_native_method_bind_events(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(30usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_generate_native_method_bind_events(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(30usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_generate_garbage_collection_events(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(31usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_generate_garbage_collection_events(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(31usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_generate_object_free_events(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(32usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_generate_object_free_events(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(32usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_force_early_return(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(33usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_force_early_return(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(33usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_get_owned_monitor_stack_depth_info(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(34usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_get_owned_monitor_stack_depth_info(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(34usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_get_constant_pool(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(35usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_get_constant_pool(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(35usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_set_native_method_prefix(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(36usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_set_native_method_prefix(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(36usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_retransform_classes(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(37usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_retransform_classes(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(37usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_retransform_any_class(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(38usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_retransform_any_class(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(38usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_generate_resource_exhaustion_heap_events(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(39usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_generate_resource_exhaustion_heap_events(
        &mut self,
        val: c_uint,
    ) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(39usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn can_generate_resource_exhaustion_threads_events(&self) -> c_uint {
        unsafe { mem::transmute(self._bitfield_1.get(40usize, 1u8) as u32) }
    }

    #[inline]
    pub fn set_can_generate_resource_exhaustion_threads_events(
        &mut self,
        val: c_uint,
    ) {
        unsafe {
            let val: u32 = mem::transmute(val);
            self._bitfield_1.set(40usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn newBitfield(
        can_tag_objects: c_uint,
        can_generate_field_modification_events: c_uint,
        can_generate_field_access_events: c_uint,
        can_get_bytecodes: c_uint,
        can_get_synthetic_attribute: c_uint,
        can_get_owned_monitor_info: c_uint,
        can_get_current_contended_monitor: c_uint,
        can_get_monitor_info: c_uint,
        can_pop_frame: c_uint,
        can_redefine_classes: c_uint,
        can_signal_thread: c_uint,
        can_get_source_file_name: c_uint,
        can_get_line_numbers: c_uint,
        can_get_source_debug_extension: c_uint,
        can_access_local_variables: c_uint,
        can_maintain_original_method_order: c_uint,
        can_generate_single_step_events: c_uint,
        can_generate_exception_events: c_uint,
        can_generate_frame_pop_events: c_uint,
        can_generate_breakpoint_events: c_uint,
        can_suspend: c_uint,
        can_redefine_any_class: c_uint,
        can_get_current_thread_cpu_time: c_uint,
        can_get_thread_cpu_time: c_uint,
        can_generate_method_entry_events: c_uint,
        can_generate_method_exit_events: c_uint,
        can_generate_all_class_hook_events: c_uint,
        can_generate_compiled_method_load_events: c_uint,
        can_generate_monitor_events: c_uint,
        can_generate_vm_object_alloc_events: c_uint,
        can_generate_native_method_bind_events: c_uint,
        can_generate_garbage_collection_events: c_uint,
        can_generate_object_free_events: c_uint,
        can_force_early_return: c_uint,
        can_get_owned_monitor_stack_depth_info: c_uint,
        can_get_constant_pool: c_uint,
        can_set_native_method_prefix: c_uint,
        can_retransform_classes: c_uint,
        can_retransform_any_class: c_uint,
        can_generate_resource_exhaustion_heap_events: c_uint,
        can_generate_resource_exhaustion_threads_events: c_uint,
    ) -> BitfieldUnit<[u8; 16usize], u8> {
        let mut unit: BitfieldUnit<[u8; 16usize], u8> =
            Default::default();
        unit.set(0usize, 1u8, {
            let can_tag_objects: u32 = unsafe { mem::transmute(can_tag_objects) };
            can_tag_objects as u64
        });
        unit.set(1usize, 1u8, {
            let can_generate_field_modification_events: u32 =
                unsafe { mem::transmute(can_generate_field_modification_events) };
            can_generate_field_modification_events as u64
        });
        unit.set(2usize, 1u8, {
            let can_generate_field_access_events: u32 =
                unsafe { mem::transmute(can_generate_field_access_events) };
            can_generate_field_access_events as u64
        });
        unit.set(3usize, 1u8, {
            let can_get_bytecodes: u32 = unsafe { mem::transmute(can_get_bytecodes) };
            can_get_bytecodes as u64
        });
        unit.set(4usize, 1u8, {
            let can_get_synthetic_attribute: u32 =
                unsafe { mem::transmute(can_get_synthetic_attribute) };
            can_get_synthetic_attribute as u64
        });
        unit.set(5usize, 1u8, {
            let can_get_owned_monitor_info: u32 =
                unsafe { mem::transmute(can_get_owned_monitor_info) };
            can_get_owned_monitor_info as u64
        });
        unit.set(6usize, 1u8, {
            let can_get_current_contended_monitor: u32 =
                unsafe { mem::transmute(can_get_current_contended_monitor) };
            can_get_current_contended_monitor as u64
        });
        unit.set(7usize, 1u8, {
            let can_get_monitor_info: u32 = unsafe { mem::transmute(can_get_monitor_info) };
            can_get_monitor_info as u64
        });
        unit.set(8usize, 1u8, {
            let can_pop_frame: u32 = unsafe { mem::transmute(can_pop_frame) };
            can_pop_frame as u64
        });
        unit.set(9usize, 1u8, {
            let can_redefine_classes: u32 = unsafe { mem::transmute(can_redefine_classes) };
            can_redefine_classes as u64
        });
        unit.set(10usize, 1u8, {
            let can_signal_thread: u32 = unsafe { mem::transmute(can_signal_thread) };
            can_signal_thread as u64
        });
        unit.set(11usize, 1u8, {
            let can_get_source_file_name: u32 =
                unsafe { mem::transmute(can_get_source_file_name) };
            can_get_source_file_name as u64
        });
        unit.set(12usize, 1u8, {
            let can_get_line_numbers: u32 = unsafe { mem::transmute(can_get_line_numbers) };
            can_get_line_numbers as u64
        });
        unit.set(13usize, 1u8, {
            let can_get_source_debug_extension: u32 =
                unsafe { mem::transmute(can_get_source_debug_extension) };
            can_get_source_debug_extension as u64
        });
        unit.set(14usize, 1u8, {
            let can_access_local_variables: u32 =
                unsafe { mem::transmute(can_access_local_variables) };
            can_access_local_variables as u64
        });
        unit.set(15usize, 1u8, {
            let can_maintain_original_method_order: u32 =
                unsafe { mem::transmute(can_maintain_original_method_order) };
            can_maintain_original_method_order as u64
        });
        unit.set(16usize, 1u8, {
            let can_generate_single_step_events: u32 =
                unsafe { mem::transmute(can_generate_single_step_events) };
            can_generate_single_step_events as u64
        });
        unit.set(17usize, 1u8, {
            let can_generate_exception_events: u32 =
                unsafe { mem::transmute(can_generate_exception_events) };
            can_generate_exception_events as u64
        });
        unit.set(18usize, 1u8, {
            let can_generate_frame_pop_events: u32 =
                unsafe { mem::transmute(can_generate_frame_pop_events) };
            can_generate_frame_pop_events as u64
        });
        unit.set(19usize, 1u8, {
            let can_generate_breakpoint_events: u32 =
                unsafe { mem::transmute(can_generate_breakpoint_events) };
            can_generate_breakpoint_events as u64
        });
        unit.set(20usize, 1u8, {
            let can_suspend: u32 = unsafe { mem::transmute(can_suspend) };
            can_suspend as u64
        });
        unit.set(21usize, 1u8, {
            let can_redefine_any_class: u32 =
                unsafe { mem::transmute(can_redefine_any_class) };
            can_redefine_any_class as u64
        });
        unit.set(22usize, 1u8, {
            let can_get_current_thread_cpu_time: u32 =
                unsafe { mem::transmute(can_get_current_thread_cpu_time) };
            can_get_current_thread_cpu_time as u64
        });
        unit.set(23usize, 1u8, {
            let can_get_thread_cpu_time: u32 =
                unsafe { mem::transmute(can_get_thread_cpu_time) };
            can_get_thread_cpu_time as u64
        });
        unit.set(24usize, 1u8, {
            let can_generate_method_entry_events: u32 =
                unsafe { mem::transmute(can_generate_method_entry_events) };
            can_generate_method_entry_events as u64
        });
        unit.set(25usize, 1u8, {
            let can_generate_method_exit_events: u32 =
                unsafe { mem::transmute(can_generate_method_exit_events) };
            can_generate_method_exit_events as u64
        });
        unit.set(26usize, 1u8, {
            let can_generate_all_class_hook_events: u32 =
                unsafe { mem::transmute(can_generate_all_class_hook_events) };
            can_generate_all_class_hook_events as u64
        });
        unit.set(27usize, 1u8, {
            let can_generate_compiled_method_load_events: u32 =
                unsafe { mem::transmute(can_generate_compiled_method_load_events) };
            can_generate_compiled_method_load_events as u64
        });
        unit.set(28usize, 1u8, {
            let can_generate_monitor_events: u32 =
                unsafe { mem::transmute(can_generate_monitor_events) };
            can_generate_monitor_events as u64
        });
        unit.set(29usize, 1u8, {
            let can_generate_vm_object_alloc_events: u32 =
                unsafe { mem::transmute(can_generate_vm_object_alloc_events) };
            can_generate_vm_object_alloc_events as u64
        });
        unit.set(30usize, 1u8, {
            let can_generate_native_method_bind_events: u32 =
                unsafe { mem::transmute(can_generate_native_method_bind_events) };
            can_generate_native_method_bind_events as u64
        });
        unit.set(31usize, 1u8, {
            let can_generate_garbage_collection_events: u32 =
                unsafe { mem::transmute(can_generate_garbage_collection_events) };
            can_generate_garbage_collection_events as u64
        });
        unit.set(32usize, 1u8, {
            let can_generate_object_free_events: u32 =
                unsafe { mem::transmute(can_generate_object_free_events) };
            can_generate_object_free_events as u64
        });
        unit.set(33usize, 1u8, {
            let can_force_early_return: u32 =
                unsafe { mem::transmute(can_force_early_return) };
            can_force_early_return as u64
        });
        unit.set(34usize, 1u8, {
            let can_get_owned_monitor_stack_depth_info: u32 =
                unsafe { mem::transmute(can_get_owned_monitor_stack_depth_info) };
            can_get_owned_monitor_stack_depth_info as u64
        });
        unit.set(35usize, 1u8, {
            let can_get_constant_pool: u32 =
                unsafe { mem::transmute(can_get_constant_pool) };
            can_get_constant_pool as u64
        });
        unit.set(36usize, 1u8, {
            let can_set_native_method_prefix: u32 =
                unsafe { mem::transmute(can_set_native_method_prefix) };
            can_set_native_method_prefix as u64
        });
        unit.set(37usize, 1u8, {
            let can_retransform_classes: u32 =
                unsafe { mem::transmute(can_retransform_classes) };
            can_retransform_classes as u64
        });
        unit.set(38usize, 1u8, {
            let can_retransform_any_class: u32 =
                unsafe { mem::transmute(can_retransform_any_class) };
            can_retransform_any_class as u64
        });
        unit.set(39usize, 1u8, {
            let can_generate_resource_exhaustion_heap_events: u32 =
                unsafe { mem::transmute(can_generate_resource_exhaustion_heap_events) };
            can_generate_resource_exhaustion_heap_events as u64
        });
        unit.set(40usize, 1u8, {
            let can_generate_resource_exhaustion_threads_events: u32 =
                unsafe { mem::transmute(can_generate_resource_exhaustion_threads_events) };
            can_generate_resource_exhaustion_threads_events as u64
        });

        unit
    }

    #[inline]
    pub fn newEmptyBitfield() -> BitfieldUnit<[u8; 16usize], u8> {
        jvmtiCapabilities::newBitfield(0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                       0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                       0)
    }
}