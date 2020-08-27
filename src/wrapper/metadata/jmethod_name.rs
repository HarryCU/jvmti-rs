use crate::objects::JvmtiString;
use crate::JSignature;

#[derive(Clone, Debug)]
pub struct JMethodName<'a> {
    pub name: String,
    pub signature: JSignature<'a>,
}

impl<'a> JMethodName<'a> {
    pub fn new(name: JvmtiString, signature: JSignature<'a>) -> JMethodName<'a> {
        JMethodName {
            name: name.into(),
            signature,
        }
    }
}
