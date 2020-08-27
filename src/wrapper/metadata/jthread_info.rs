use crate::{
    objects::*,
    stringify,
};
use crate::sys::jvmtiThreadInfo;
use std::marker::PhantomData;
use std::ptr;

#[derive(Clone, Debug)]
pub struct JThreadInfo<'a> {
    internal: jvmtiThreadInfo,
    lifetime: PhantomData<&'a ()>,

    pub name: String,
    pub priority: u32,
    pub is_daemon: bool,
    pub thread_group: JThreadGroupID<'a>,
    pub context_class_loader: JObject<'a>,
}

impl<'a> JThreadInfo<'a> {
    pub fn new(info: jvmtiThreadInfo) -> JThreadInfo<'a> {
        JThreadInfo {
            internal: info,
            lifetime: PhantomData,

            name: stringify(info.name),
            priority: info.priority as u32,
            is_daemon: if info.is_daemon > 0 { true } else { false },
            thread_group: info.thread_group.into(),
            context_class_loader: info.context_class_loader.into(),
        }
    }

    pub fn empty_raw() -> jvmtiThreadInfo {
        jvmtiThreadInfo {
            name: ptr::null_mut(),
            priority: 0,
            is_daemon: 0,
            thread_group: ptr::null_mut(),
            context_class_loader: ptr::null_mut(),
        }
    }
}

impl<'a> ::std::ops::Deref for JThreadInfo<'a> {
    type Target = jvmtiThreadInfo;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}
