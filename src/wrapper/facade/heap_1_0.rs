use std::ffi::c_void;

use crate::{sys::*, objects::*, errors::*, Transform, JVMTIEnvFacade};

impl<'a> JVMTIEnvFacade<'a> {
    pub fn iterate_over_instances_of_class<K>(&self, class: K, object_filter: jvmtiHeapObjectFilter,
                                              heap_object_callback: jvmtiHeapObjectCallback,
                                              user_data: *const c_void) -> Result<()>
        where
            K: Transform<'a, JClass<'a>> {
        self.jvmti_rust().iterate_over_instances_of_class(self.jni_rust(), class, object_filter, heap_object_callback, user_data)
    }
}
