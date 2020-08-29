use crate::{builder::*, JVMTIEnv, objects::*};
use crate::sys;
use crate::sys::jthread;

#[derive(Clone, Debug)]
pub struct JMonitorUsage<'a> {
    internal: sys::jvmtiMonitorUsage,

    pub owner: JThreadID<'a>,
    pub entry_count: u32,
    pub waiters: Vec<JThreadID<'a>>,
    pub notify_waiters: Vec<JThreadID<'a>>,
}

impl<'a> JMonitorUsage<'a> {
    pub fn new<'b: 'a>(usage: sys::jvmtiMonitorUsage, jvmti: &'b JVMTIEnv<'a>) -> Self {
        let waiters: MutAutoDeallocateObjectArrayBuilder<jthread> = MutAutoDeallocateObjectArrayBuilder::create(usage.waiter_count, usage.waiters);
        let notify_waiters: MutAutoDeallocateObjectArrayBuilder<jthread> = MutAutoDeallocateObjectArrayBuilder::create(usage.notify_waiter_count, usage.notify_waiters);

        return JMonitorUsage {
            internal: usage,

            owner: usage.owner.into(),
            entry_count: usage.entry_count as u32,
            waiters: waiters.build(jvmti),
            notify_waiters: notify_waiters.build(jvmti),
        };
    }
}

impl<'a> ::std::ops::Deref for JMonitorUsage<'a> {
    type Target = sys::jvmtiMonitorUsage;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}