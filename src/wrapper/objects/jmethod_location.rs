use crate::sys::jlocation;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct JMethodLocation<'a> {
    lifetime: PhantomData<&'a ()>,

    pub start_location: jlocation,
    pub end_location: jlocation,
}

impl<'a> JMethodLocation<'a> {
    pub fn new(start_location: jlocation,
               end_location: jlocation) -> JMethodLocation<'a> {
        JMethodLocation {
            lifetime: PhantomData,

            start_location: start_location.into(),
            end_location: end_location.into(),
        }
    }
}