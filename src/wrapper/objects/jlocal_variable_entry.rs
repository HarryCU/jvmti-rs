use crate::wrapper::*;

use crate::sys::{jvmtiLocalVariableEntry, jlocation};

#[derive(Clone)]
pub struct JLocalVariableEntry<'a> {
    pub start_location: jlocation,
    pub length: i32,
    pub name: String,
    pub signature: JSignature<'a>,
    pub slot: i32,
}

impl<'a> JLocalVariableEntry<'a> {
    pub fn new<'b>(env: &JVMTIEnv<'b>, entry: &jvmtiLocalVariableEntry) -> Result<JLocalVariableEntry<'a>> {
        let signature = JString::with_pointer(entry.signature, env);
        let generic = JString::with_pointer(entry.generic_signature, env);
        let signature = JSignature::new(signature, generic)?;

        Ok(JLocalVariableEntry {
            start_location: entry.start_location.into(),
            length: entry.length,
            name: stringify(entry.name),
            signature,
            slot: entry.slot,
        })
    }
}