use std::marker::PhantomData;

use crate::sys::{jthreadGroup};

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct JThreadGroupID<'a> {
    internal: jthreadGroup,
    lifetime: PhantomData<&'a ()>,
}

impl<'a> From<jthreadGroup> for JThreadGroupID<'a> {
    fn from(other: jthreadGroup) -> Self {
        JThreadGroupID {
            internal: other,
            lifetime: PhantomData,
        }
    }
}

impl<'a> From<JThreadGroupID<'a>> for jthreadGroup {
    fn from(value: JThreadGroupID<'a>) -> Self {
        value.internal
    }
}

impl<'a> From<&JThreadGroupID<'a>> for jthreadGroup {
    fn from(value: &JThreadGroupID<'a>) -> Self {
        value.internal
    }
}

impl<'a> ::std::ops::Deref for JThreadGroupID<'a> {
    type Target = jthreadGroup;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a> JThreadGroupID<'a> {
    /// Creates a new null threadGroup
    pub fn null() -> JThreadGroupID<'a> {
        (::std::ptr::null_mut() as jthreadGroup).into()
    }
}
