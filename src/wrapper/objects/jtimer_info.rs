use crate::sys::{jlong, jvmtiTimerInfo, jvmtiTimerKind, jboolean};
use crate::{JvmtiTimerKind, to_bool};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct JTimerInfo<'a> {
    internal: jvmtiTimerInfo,
    lifetime: PhantomData<&'a ()>,

    pub max_value: jlong,
    pub may_skip_forward: bool,
    pub may_skip_backward: bool,
    pub kind: JvmtiTimerKind,
    pub reserved1: jlong,
    pub reserved2: jlong,
}

impl<'a> JTimerInfo<'a> {
    pub fn empty_raw() -> jvmtiTimerInfo {
        jvmtiTimerInfo {
            max_value: 0 as jlong,
            may_skip_forward: 0 as jboolean,
            may_skip_backward: 0 as jboolean,
            kind: 0 as jvmtiTimerKind,
            reserved1: 0 as jlong,
            reserved2: 0 as jlong,
        }
    }
}

impl<'a> From<jvmtiTimerInfo> for JTimerInfo<'a> {
    fn from(info: jvmtiTimerInfo) -> Self {
        JTimerInfo {
            internal: info,
            lifetime: PhantomData,

            max_value: info.max_value,
            may_skip_forward: to_bool(info.may_skip_forward),
            may_skip_backward: to_bool(info.may_skip_backward),
            kind: info.kind.into(),
            reserved1: info.reserved1,
            reserved2: info.reserved2,
        }
    }
}

impl<'a> ::std::ops::Deref for JTimerInfo<'a> {
    type Target = jvmtiTimerInfo;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}