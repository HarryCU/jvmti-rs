use crate::sys::{jvmtiMonitorStackDepthInfo, JObject};

#[derive(Debug)]
pub struct JMonitorStackDepthInfo<'a> {
    internal: jvmtiMonitorStackDepthInfo,

    pub monitor: JObject<'a>,
    pub stack_depth: u32,
}


impl<'a> From<jvmtiMonitorStackDepthInfo> for JMonitorStackDepthInfo<'a> {
    fn from(info: jvmtiMonitorStackDepthInfo) -> Self {
        JMonitorStackDepthInfo {
            internal: info,

            monitor: info.monitor.into(),
            stack_depth: info.stack_depth as u32,
        }
    }
}

impl<'a> ::std::ops::Deref for JMonitorStackDepthInfo<'a> {
    type Target = jvmtiMonitorStackDepthInfo;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}
