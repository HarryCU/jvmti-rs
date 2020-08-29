use std::ptr;
use jni::sys::jobject;
use crate::{
    errors::*,
    JVMTIEnv,
    objects::*,
    sys::*,
};

impl<'a> JVMTIEnv<'a> {
    pub fn get_local_instance(&self, thread: &JThreadID, depth: jint) -> Result<Option<JObject>> {
        let res = jvmti_call_ptr_result!(self.jvmti_raw(), jobject,
            GetLocalInstance,
            thread.into(),
            depth
        );
        if res.is_null() {
            return Ok(None);
        }
        Ok(Some(res.into()))
    }

    pub fn get_local_object(&self, thread: &JThreadID, depth: jint, slot: jint) -> Result<Option<JObject>> {
        let res = jvmti_call_ptr_result!(self.jvmti_raw(), jobject,
            GetLocalObject,
            thread.into(),
            depth,
            slot
        );
        if res.is_null() {
            return Ok(None);
        }
        Ok(Some(res.into()))
    }

    pub fn get_local_int(&self, thread: &JThreadID, depth: jint, slot: jint) -> Result<jint> {
        Ok(jvmti_call_number_result!(self.jvmti_raw(), jint,
            GetLocalInt,
            thread.into(),
            depth,
            slot
        ))
    }

    pub fn get_local_long(&self, thread: &JThreadID, depth: jint, slot: jint) -> Result<jlong> {
        Ok(jvmti_call_number_result!(self.jvmti_raw(), jlong,
            GetLocalLong,
            thread.into(),
            depth,
            slot
       ))
    }

    pub fn get_local_float(&self, thread: &JThreadID, depth: jint, slot: jint) -> Result<jfloat> {
        Ok(jvmti_call_number_result!(self.jvmti_raw(), jfloat,
            GetLocalFloat,
            thread.into(),
            depth,
            slot
       ))
    }

    pub fn get_local_double(&self, thread: &JThreadID, depth: jint, slot: jint) -> Result<jdouble> {
        Ok(jvmti_call_number_result!(self.jvmti_raw(), jdouble,
            GetLocalDouble,
            thread.into(),
            depth,
            slot
       ))
    }

    pub fn set_local_object(&self, thread: &JThreadID, depth: jint, slot: jint, obj: &Option<JObject>) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), SetLocalObject,
            thread.into(),
            depth,
            slot,
            obj.map_or_else(|| ptr::null_mut(), |e| e.into_inner())
        )
    }

    pub fn set_local_int(&self, thread: &JThreadID, depth: jint, slot: jint, value: jint) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), SetLocalInt,
            thread.into(),
            depth,
            slot,
            value
        )
    }

    pub fn set_local_long(&self, thread: &JThreadID, depth: jint, slot: jint, value: jlong) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), SetLocalLong,
            thread.into(),
            depth,
            slot,
            value
        )
    }

    pub fn set_local_float(&self, thread: &JThreadID, depth: jint, slot: jint, value: jfloat) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), SetLocalFloat,
            thread.into(),
            depth,
            slot,
            value
        )
    }

    pub fn set_local_double(&self, thread: &JThreadID, depth: jint, slot: jint, value: jdouble) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), SetLocalDouble,
            thread.into(),
            depth,
            slot,
            value
        )
    }
}
