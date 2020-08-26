use crate::sys::jmemory;

pub trait JDeallocate<'a> {
    fn as_deallocate_ptr(&self) -> jmemory;
}