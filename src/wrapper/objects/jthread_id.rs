use std::marker::PhantomData;

use crate::sys::{
    jobject,
    jthread,
};

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct JThreadID<'a> {
    internal: jthread,
    lifetime: PhantomData<&'a ()>,
}

impl<'a> From<jthread> for JThreadID<'a> {
    fn from(thread: jthread) -> Self {
        JThreadID {
            internal: thread,
            lifetime: PhantomData,
        }
    }
}

impl<'a> From<&JThreadID<'a>> for jthread {
    fn from(other: &JThreadID<'a>) -> Self {
        other.internal
    }
}

impl<'a> From<JThreadID<'a>> for jthread {
    fn from(other: JThreadID<'a>) -> Self {
        other.internal
    }
}

impl<'a> ::std::ops::Deref for JThreadID<'a> {
    type Target = jthread;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a> JThreadID<'a> {
    pub fn new(obj: jobject) -> JThreadID<'a> {
        JThreadID {
            internal: obj,
            lifetime: PhantomData,
        }
    }

    /// Creates a new null thread
    pub fn null() -> JThreadID<'a> {
        (::std::ptr::null_mut() as jthread).into()
    }
}
