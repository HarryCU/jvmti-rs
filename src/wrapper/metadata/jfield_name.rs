use crate::wrapper::*;
use std::marker::PhantomData;
use crate::sys::JFieldID;

#[derive(Clone, Debug)]
pub struct JFieldName<'a> {
    pub name: String,
    pub signature: JSignature<'a>,
}

impl<'a> JFieldName<'a> {
    pub fn new(name: JString, signature: JSignature<'a>) -> JFieldName<'a> {
        JFieldName {
            name: name.into(),
            signature,
        }
    }
}
