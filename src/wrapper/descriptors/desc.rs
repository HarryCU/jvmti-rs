use crate::errors::*;
use crate::JVMTIEnv;

/// see https://github.com/jni-rs/jni-rs/blob/master/src/wrapper/descriptors/desc.rs
///
/// Trait for things that can be looked up through the JNI via a descriptor.
/// This will be something like the fully-qualified class name
/// `java/lang/String` or a tuple containing a class descriptor, method name,
/// and method signature. For convenience, this is also implemented for the
/// concrete types themselves in addition to their descriptors.
pub trait JvmtiDesc<'a, T> {
    /// Different
    /// Look up the concrete type from the JVM.
    fn lookup(self, _: &JVMTIEnv<'a>) -> Result<T>;
}

impl<'a, T> JvmtiDesc<'a, T> for T {
    /// Different
    fn lookup(self, _: &JVMTIEnv<'a>) -> Result<T> {
        Ok(self)
    }
}
