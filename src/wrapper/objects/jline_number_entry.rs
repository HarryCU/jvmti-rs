use crate::wrapper::JLocation;
use crate::sys::{jint, jvmtiLineNumberEntry};

#[derive(Copy, Clone)]
pub struct JLineNumberEntry<'a> {
    internal: jvmtiLineNumberEntry,

    pub start_location: JLocation<'a>,
    pub line_number: i32,
}

impl<'a> From<jvmtiLineNumberEntry> for JLineNumberEntry<'a> {
    fn from(info: jvmtiLineNumberEntry) -> Self {
        JLineNumberEntry {
            internal: info,

            start_location: info.start_location.into(),
            line_number: info.line_number,
        }
    }
}

impl<'a> ::std::ops::Deref for JLineNumberEntry<'a> {
    type Target = jvmtiLineNumberEntry;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}