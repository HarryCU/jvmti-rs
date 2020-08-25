use std::os::raw::c_uchar;
use crate::sys::jmemory;

pub trait JDeallocate<'a> {
    fn as_ptr(&self) -> jmemory;
}