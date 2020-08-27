use crate::*;
use crate::objects::JvmtiString;

#[derive(Clone, Debug)]
pub struct JFieldName<'a> {
    pub name: String,
    pub signature: JSignature<'a>,
}

impl<'a> JFieldName<'a> {
    pub fn new(name: JvmtiString, signature: JSignature<'a>) -> JFieldName<'a> {
        JFieldName {
            name: name.into(),
            signature,
        }
    }
}
