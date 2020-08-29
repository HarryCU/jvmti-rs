use std::ffi::c_void;

use crate::{sys::*, errors::*, objects::*, JVMTIEnv,  Transform};

impl<'a> JVMTIEnv<'a> {
    pub fn follow_references<K>(&self, jni: &jni::JNIEnv<'a>, heap_filter: jint, class: K,
                                initial_object: &JObject,
                                callbacks: &Vec<jvmtiHeapCallbacks>,
                                user_data: *const c_void,
    ) -> Result<()>
        where
            K: Transform<'a, JClass<'a>>, {
        let klass = class.transform(jni)?;

        jvmti_call!(self.jvmti_raw(), FollowReferences,
            heap_filter,
            klass.into_inner(),
            initial_object.into_inner(),
            callbacks.as_ptr(),
            user_data
        )
    }

    pub fn iterate_through_heap<K>(&self, jni: &jni::JNIEnv<'a>, heap_filter: jint, class: K,
                                   callbacks: &Vec<jvmtiHeapCallbacks>,
                                   user_data: *const c_void,
    ) -> Result<()>
        where
            K: Transform<'a, JClass<'a>>, {
        let klass = class.transform(jni)?;

        jvmti_call!(self.jvmti_raw(), IterateThroughHeap,
            heap_filter,
            klass.into_inner(),
            callbacks.as_ptr(),
            user_data
        )
    }
}
