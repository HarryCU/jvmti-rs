use crate::*;
use crate::{objects::*, errors::*};

impl<'a, 'b> JvmtiDesc<'a, JClass<'a>> for JObject<'b> {
    fn lookup(self, env: &JVMTIEnv<'a>) -> Result<JClass<'a>> {
        let jni = env.get_jni()?;
        jni.get_object_class(self)
            .map_err(jni_lookup_error)
    }
}
