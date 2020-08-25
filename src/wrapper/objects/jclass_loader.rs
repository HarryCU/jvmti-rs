use jni_sys::jobject;
use std::marker::PhantomData;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct JClassLoader<'a> {
    internal: jobject,
    lifetime: PhantomData<&'a ()>,
}

impl<'a> From<jobject> for JClassLoader<'a> {
    fn from(location: jobject) -> Self {
        JClassLoader {
            internal: location,
            lifetime: PhantomData,
        }
    }
}

impl<'a> From<JClassLoader<'a>> for jobject {
    fn from(location: JClassLoader<'a>) -> Self {
        location.internal
    }
}

impl<'a> ::std::ops::Deref for JClassLoader<'a> {
    type Target = jobject;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}