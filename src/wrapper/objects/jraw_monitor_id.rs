use crate::sys::jrawMonitorID;
use std::marker::PhantomData;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct JRawMonitorID<'a> {
    internal: jrawMonitorID,
    lifetime: PhantomData<&'a ()>,
}

impl<'a> From<jrawMonitorID> for JRawMonitorID<'a> {
    fn from(monitor_id: jrawMonitorID) -> Self {
        JRawMonitorID {
            internal: monitor_id,
            lifetime: PhantomData,
        }
    }
}

impl<'a> From<JRawMonitorID<'a>> for jrawMonitorID {
    fn from(monitor_id: JRawMonitorID<'a>) -> Self {
        monitor_id.internal
    }
}

impl<'a> From<&JRawMonitorID<'a>> for jrawMonitorID {
    fn from(monitor_id: &JRawMonitorID<'a>) -> Self {
        monitor_id.internal
    }
}

impl<'a> ::std::ops::Deref for JRawMonitorID<'a> {
    type Target = jrawMonitorID;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}