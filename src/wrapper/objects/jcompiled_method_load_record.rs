use crate::objects::JMethodID;

#[derive(Debug)]
pub enum JCompiledMethodLoadRecord<'a> {
    Inline { stack_infos: Vec<JCompiledMethodLoadRecordStackInfo<'a>> },
    Dummy,
}

#[derive(Debug)]
pub struct JCompiledMethodLoadRecordStackInfo<'a> {
    pub pc_address: usize,
    pub stack_frames: Vec<JCompiledMethodLoadRecordStackFrame<'a>>,
}

#[derive(Debug)]
pub struct JCompiledMethodLoadRecordStackFrame<'a> {
    pub method: JMethodID<'a>,
    pub byte_code_index: i32,
}