use crate::{objects::*, stringify};

use std::marker::PhantomData;
use crate::sys::jvmtiThreadGroupInfo;

#[derive(Clone, Debug)]
pub struct JThreadGroupInfo<'a> {
    internal: jvmtiThreadGroupInfo,
    lifetime: PhantomData<&'a ()>,

    pub parent: JThreadGroupID<'a>,
    pub name: String,
    pub max_priority: u32,
    pub is_daemon: bool,
}

impl<'a> JThreadGroupInfo<'a> {
    pub fn new(info: jvmtiThreadGroupInfo) -> JThreadGroupInfo<'a> {
        JThreadGroupInfo {
            internal: info,
            lifetime: PhantomData,

            parent: info.parent.into(),
            name: stringify(info.name),
            max_priority: info.max_priority as u32,
            is_daemon: if info.is_daemon > 0 { true } else { false },
        }
    }
}

impl<'a> ::std::ops::Deref for JThreadGroupInfo<'a> {
    type Target = jvmtiThreadGroupInfo;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}
