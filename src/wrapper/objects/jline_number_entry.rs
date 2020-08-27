use crate::sys::{jvmtiLineNumberEntry, jlocation};
use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub struct JLineNumberEntry<'a> {
    internal: jvmtiLineNumberEntry,
    lifetime: PhantomData<&'a ()>,

    pub start_location: jlocation,
    pub line_number: i32,
}

impl<'a> From<jvmtiLineNumberEntry> for JLineNumberEntry<'a> {
    fn from(info: jvmtiLineNumberEntry) -> Self {
        JLineNumberEntry {
            internal: info,
            lifetime: PhantomData,

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