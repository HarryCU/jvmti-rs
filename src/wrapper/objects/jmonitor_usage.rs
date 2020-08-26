use crate::wrapper::{JThreadID, MutObjectArrayBuilder, Builder};
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

impl<'a> From<sys::jvmtiMonitorUsage> for JMonitorUsage<'a> {
    fn from(usage: sys::jvmtiMonitorUsage) -> Self {
        let waiters: MutObjectArrayBuilder<jthread> = MutObjectArrayBuilder::create(usage.waiter_count, usage.waiters);
        let notify_waiters: MutObjectArrayBuilder<jthread> = MutObjectArrayBuilder::create(usage.notify_waiter_count, usage.notify_waiters);

        return JMonitorUsage {
            internal: usage,

            owner: usage.owner.into(),
            entry_count: usage.entry_count as u32,
            waiters: waiters.build(),
            notify_waiters: notify_waiters.build(),
        };
    }
}

impl<'a> ::std::ops::Deref for JMonitorUsage<'a> {
    type Target = sys::jvmtiMonitorUsage;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}