use crate::wrapper::JLocation;
use crate::sys::jlocation;

#[derive(Clone, Debug)]
pub struct JMethodLocation<'a> {
    pub start_location: JLocation<'a>,
    pub end_location: JLocation<'a>,
}

impl<'a> JMethodLocation<'a> {
    pub fn new(start_location: jlocation,
               end_location: jlocation) -> JMethodLocation<'a> {
        JMethodLocation {
            start_location: start_location.into(),
            end_location: end_location.into(),
        }
    }
}