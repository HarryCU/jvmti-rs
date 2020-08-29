use std::ffi::c_void;

use crate::{sys::*, errors::*, objects::*, Transform, JVMTIFacadeEnv};

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn follow_references<K>(&self, heap_filter: jint, class: K, initial_object: &JObject<'a>,
                                callbacks: &Vec<jvmtiHeapCallbacks>,
                                user_data: *const c_void) -> Result<()>
        where
            K: Transform<'a, JClass<'a>> {
        self.jvmti_rust().follow_references(self.jni_rust(), heap_filter, class, initial_object, callbacks, user_data)
    }

    pub fn iterate_through_heap<K>(&self, heap_filter: jint, class: K,
                                   callbacks: &Vec<jvmtiHeapCallbacks>,
                                   user_data: *const c_void) -> Result<()>
        where
            K: Transform<'a, JClass<'a>> {
        self.jvmti_rust().iterate_through_heap(self.jni_rust(), heap_filter, class, callbacks, user_data)
    }
}
