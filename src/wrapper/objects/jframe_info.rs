use crate::objects::JMethodID;
use crate::sys::{jvmtiFrameInfo, jlocation};

#[derive(Clone, Debug)]
pub struct JFrameInfo<'a> {
    internal: jvmtiFrameInfo,

    pub method: JMethodID<'a>,
    pub location: jlocation,
}

impl<'a> From<jvmtiFrameInfo> for JFrameInfo<'a> {
    fn from(info: jvmtiFrameInfo) -> Self {
        JFrameInfo {
            internal: info,

            method: info.method.into(),
            location: info.location,
        }
    }
}

impl<'a> ::std::ops::Deref for JFrameInfo<'a> {
    type Target = jvmtiFrameInfo;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}