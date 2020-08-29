use crate::{builder::*, JVMTIEnv, JvmtiError, objects::*, stringify, sys::*};

#[derive(Debug)]
pub struct JExtensionFunctionInfo<'a> {
    internal: jvmtiExtensionFunctionInfo,

    pub func: jvmtiExtensionFunction,
    pub id: String,
    pub short_description: String,
    pub params: Vec<JParamInfo<'a>>,
    pub errors: Vec<JvmtiError>,
}

impl<'a> JExtensionFunctionInfo<'a> {
    pub fn new<'b: 'a>(info: jvmtiExtensionFunctionInfo, jvmti: &'b JVMTIEnv<'a>) -> JExtensionFunctionInfo<'a> {
        let params_builder: MutAutoDeallocateObjectArrayBuilder<jvmtiParamInfo> = MutAutoDeallocateObjectArrayBuilder::create(info.param_count, info.params);
        let params: Vec<JParamInfo<'a>> = params_builder.build(jvmti);
        let errors_builder: MutObjectArrayBuilder<jvmtiError> = MutObjectArrayBuilder::create(info.error_count, info.errors);
        let errors: Vec<JvmtiError> = errors_builder.build();

        JExtensionFunctionInfo {
            internal: info,

            func: info.func,
            id: stringify(info.id),
            short_description: stringify(info.short_description),
            params,
            errors,
        }
    }
}

impl<'a> ::std::ops::Deref for JExtensionFunctionInfo<'a> {
    type Target = jvmtiExtensionFunctionInfo;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}
