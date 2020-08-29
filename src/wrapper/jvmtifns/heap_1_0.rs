use std::ffi::c_void;

use crate::{errors::*, JVMTIEnv, objects::*, sys::*};

impl<'a> JVMTIEnv<'a> {
    pub fn iterate_over_objects_reachable_from_object(&self, object: &JObject, callback: jvmtiObjectReferenceCallback, user_data: *const c_void) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), IterateOverObjectsReachableFromObject,
            object.into_inner(),
            callback,
            user_data
        )
    }

    pub fn iterate_over_reachable_objects(&self, heap_root_callback: jvmtiHeapRootCallback,
                                          stack_ref_callback: jvmtiStackReferenceCallback,
                                          object_ref_callback: jvmtiObjectReferenceCallback,
                                          user_data: *const c_void,
    ) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), IterateOverReachableObjects,
            heap_root_callback,
            stack_ref_callback,
            object_ref_callback,
            user_data
        )
    }

    pub fn iterate_over_heap(&self, object_filter: jvmtiHeapObjectFilter,
                             heap_object_callback: jvmtiHeapObjectCallback,
                             user_data: *const c_void,
    ) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), IterateOverHeap,
            object_filter,
            heap_object_callback,
            user_data
        )
    }
}
