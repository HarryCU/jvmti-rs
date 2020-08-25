use crate::wrapper::*;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct JSignature<'a> {
    lifetime: PhantomData<&'a ()>,

    pub signature: String,
    pub generic_signature: String,
}

impl<'a> JSignature<'a> {
    pub fn new(signature: JString, generic_signature: JString) -> Result<JSignature<'a>> {
        Ok(JSignature {
            lifetime: PhantomData,

            signature: signature.into(),
            generic_signature: generic_signature.into(),
        })
    }
}
