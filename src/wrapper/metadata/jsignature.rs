use crate::errors::*;
use std::marker::PhantomData;
use crate::objects::JvmtiString;

#[derive(Clone, Debug)]
pub struct JSignature<'a> {
    lifetime: PhantomData<&'a ()>,

    pub signature: String,
    pub generic_signature: String,
}

impl<'a> JSignature<'a> {
    pub fn new(signature: JvmtiString, generic_signature: JvmtiString) -> Result<JSignature<'a>> {
        Ok(JSignature {
            lifetime: PhantomData,

            signature: signature.into(),
            generic_signature: generic_signature.into(),
        })
    }
}
