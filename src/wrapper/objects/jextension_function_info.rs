use crate::sys::{jvmtiExtensionFunctionInfo, jvmtiExtensionFunction, jvmtiParamInfo, jvmtiError};
use crate::wrapper::{JParamInfo, JvmtiError, MutObjectArrayBuilder, stringify, Builder};

#[derive(Debug)]
pub struct JExtensionFunctionInfo<'a> {
    internal: jvmtiExtensionFunctionInfo,

    pub func: jvmtiExtensionFunction,
    pub id: String,
    pub short_description: String,
    pub params: Vec<JParamInfo<'a>>,
    pub errors: Vec<JvmtiError>,
}

impl<'a> From<jvmtiExtensionFunctionInfo> for JExtensionFunctionInfo<'a> {
    fn from(info: jvmtiExtensionFunctionInfo) -> Self {
        let params_builder: MutObjectArrayBuilder<jvmtiParamInfo> = MutObjectArrayBuilder::create(info.param_count, info.params);
        let params: Vec<JParamInfo<'a>> = params_builder.build();
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
