use std::ptr;

use crate::{sys::*, errors::*, objects::*, JVMTIEnv, slice_raw};

impl<'a> JVMTIEnv<'a> {
    pub fn get_tag(&self, object: &JObject) -> Result<jlong> {
        Ok(jvmti_call_number_result!(self.jvmti_raw(), jlong,
            GetTag,
            object.into_inner()
       ))
    }

    pub fn set_tag(&self, object: &JObject, tag: jlong) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), SetTag,
            object.into_inner(),
            tag
        )
    }

    pub fn get_objects_with_tags(&self, tags: &Vec<jlong>) -> Result<Vec<JTagObject>> {
        if tags.is_empty() {
            return Ok(vec![]);
        }

        let mut count: jint = 0 as jint;
        let mut object_result: *mut jobject = ptr::null_mut();
        let mut tag_result: *mut jlong = ptr::null_mut();

        let res = jvmti_call_result!( self.jvmti_raw(), GetObjectsWithTags,
            tags.len() as jint,
            tags.as_ptr(),
            &mut count,
            &mut object_result,
            &mut tag_result
        );
        jvmti_error_code_to_result(res)?;
        if count == 0 || object_result.is_null() || tag_result.is_null() {
            return Ok(vec![]);
        }
        // TODO: to be optimized
        let objects = slice_raw(object_result, count);
        let tags = slice_raw(tag_result, count);
        let mut jtag_objects: Vec<JTagObject> = Vec::with_capacity(count as usize);
        for idx in 0..(count as usize) {
            let o: jobject = objects[idx..idx + 1][0];
            let t: jlong = tags[idx..idx + 1][0];
            jtag_objects.push(JTagObject::new(o, t))
        }
        Ok(jtag_objects)
    }

    pub fn force_garbage_collection(&self) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), ForceGarbageCollection)
    }
}
