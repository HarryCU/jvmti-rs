use std::ffi::c_void;

use crate::{sys::*, objects::*, errors::*, JVMTIEnv, Transform};

impl<'a> JVMTIEnv<'a> {
    pub fn iterate_over_instances_of_class<K>(&self, jni: &jni::JNIEnv<'a>, class: K,
                                              object_filter: jvmtiHeapObjectFilter,
                                              heap_object_callback: jvmtiHeapObjectCallback,
                                              user_data: *const c_void,
    ) -> Result<()>
        where
            K: Transform<'a, JClass<'a>>, {
        let klass = class.transform(jni)?;

        jvmti_call!(self.jvmti_raw(), IterateOverInstancesOfClass,
            klass.into_inner(),
            object_filter,
            heap_object_callback,
            user_data
        )
    }
}
