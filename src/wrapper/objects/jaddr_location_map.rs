use crate::sys::jvmtiAddrLocationMap;
use std::marker::PhantomData;
use std::ffi::c_void;
use crate::wrapper::JLocation;

#[derive(Clone, Debug)]
pub struct JAddrLocationMap<'a> {
    internal: jvmtiAddrLocationMap,
    lifetime: PhantomData<&'a ()>,

    pub start_address: *const c_void,
    pub location: JLocation<'a>,
}


impl<'a> From<jvmtiAddrLocationMap> for JAddrLocationMap<'a> {
    fn from(location_map: jvmtiAddrLocationMap) -> Self {
        JAddrLocationMap {
            internal: location_map,
            lifetime: PhantomData,

            start_address: location_map.start_address,
            location: location_map.location.into(),
        }
    }
}

impl<'a> ::std::ops::Deref for JAddrLocationMap<'a> {
    type Target = jvmtiAddrLocationMap;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}