use crate::wrapper::*;
use std::marker::PhantomData;
use crate::sys::JFieldID;

#[derive(Clone, Debug)]
pub struct JMethodName<'a> {
    pub name: String,
    pub signature: JSignature<'a>,
}

impl<'a> JMethodName<'a> {
    pub fn new(name: JString, signature: JSignature<'a>) -> JMethodName<'a> {
        JMethodName {
            name: name.into(),
            signature,
        }
    }
}