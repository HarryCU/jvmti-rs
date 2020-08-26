use std::marker::PhantomData;

use crate::sys::{jlocation};

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct JLocation<'a> {
    internal: jlocation,
    lifetime: PhantomData<&'a ()>,
}

impl<'a> From<jlocation> for JLocation<'a> {
    fn from(location: jlocation) -> Self {
        JLocation {
            internal: location,
            lifetime: PhantomData,
        }
    }
}

impl<'a> From<JLocation<'a>> for jlocation {
    fn from(location: JLocation<'a>) -> Self {
        location.internal
    }
}

impl<'a> From<&JLocation<'a>> for jlocation {
    fn from(location: &JLocation<'a>) -> Self {
        location.internal
    }
}

impl<'a> ::std::ops::Deref for JLocation<'a> {
    type Target = jlocation;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}