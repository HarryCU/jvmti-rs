use crate::sys::{JMethodID, jvmtiFrameInfo, jvmtiStackInfo, jlocation};
use crate::wrapper::{JThreadID, MutObjectArrayBuilder, Builder};

#[derive(Clone)]
pub struct JStackInfo<'a> {
    internal: jvmtiStackInfo,

    pub thread: JThreadID<'a>,
    pub state: i32,
    pub frame_buffer: Vec<JFrameInfo<'a>>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JFrameInfo<'a> {
    internal: jvmtiFrameInfo,

    pub method: JMethodID<'a>,
    pub location: jlocation,
}

impl<'a> From<jvmtiStackInfo> for JStackInfo<'a> {
    fn from(stack_info: jvmtiStackInfo) -> Self {
        let builder: MutObjectArrayBuilder<jvmtiFrameInfo> = MutObjectArrayBuilder::create(stack_info.frame_count, stack_info.frame_buffer);
        let frame_buffer: Vec<JFrameInfo<'a>> = builder.build();
        return JStackInfo {
            internal: stack_info,

            thread: stack_info.thread.into(),
            state: stack_info.state,
            frame_buffer,
        };
    }
}

impl<'a> ::std::ops::Deref for JStackInfo<'a> {
    type Target = jvmtiStackInfo;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a> From<jvmtiFrameInfo> for JFrameInfo<'a> {
    fn from(frame_info: jvmtiFrameInfo) -> Self {
        return JFrameInfo {
            internal: frame_info,

            method: frame_info.method.into(),
            location: frame_info.location,
        };
    }
}

impl<'a> ::std::ops::Deref for JFrameInfo<'a> {
    type Target = jvmtiFrameInfo;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}