use crate::sys::JObject;
use jni_sys::{jlong, jobject};

pub struct JTagObject<'a> {
    pub object: JObject<'a>,
    pub tag: jlong,
}

impl<'a> JTagObject<'a> {
    pub fn new(object: jobject, tag: jlong) -> JTagObject<'a> {
        JTagObject {
            object: object.into(),
            tag,
        }
    }
}