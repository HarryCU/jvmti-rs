use std::marker::PhantomData;
use crate::sys::jint;

pub struct JClassVersionNumber<'a> {
    lifetime: PhantomData<&'a ()>,

    pub minor_version: u32,
    pub major_version: u32,
}

impl<'a> JClassVersionNumber<'a> {
    pub fn new(minor_version: jint, major_version: jint) -> JClassVersionNumber<'a> {
        JClassVersionNumber {
            lifetime: PhantomData,

            minor_version: minor_version as u32,
            major_version: major_version as u32,
        }
    }
}