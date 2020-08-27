use std::marker::PhantomData;
use crate::{sys::*, builder::*, JvmtiParamTypes, JvmtiParamKind, stringify, to_bool};

#[derive(Debug)]
pub struct JExtensionEventInfo<'a> {
    internal: jvmtiExtensionEventInfo,

    pub extension_event_index: u32,
    pub id: String,
    pub short_description: String,
    pub params: Vec<JParamInfo<'a>>,
}

impl<'a> From<jvmtiExtensionEventInfo> for JExtensionEventInfo<'a> {
    fn from(info: jvmtiExtensionEventInfo) -> Self {
        let builder: MutObjectArrayBuilder<jvmtiParamInfo> = MutObjectArrayBuilder::create(info.param_count, info.params);
        let params: Vec<JParamInfo<'a>> = builder.build();

        JExtensionEventInfo {
            internal: info,

            extension_event_index: info.extension_event_index as u32,
            id: stringify(info.id),
            short_description: stringify(info.short_description),
            params,
        }
    }
}

impl<'a> ::std::ops::Deref for JExtensionEventInfo<'a> {
    type Target = jvmtiExtensionEventInfo;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}


#[derive(Debug)]
pub struct JParamInfo<'a> {
    internal: jvmtiParamInfo,
    lifetime: PhantomData<&'a ()>,

    pub name: String,
    pub kind: JvmtiParamKind,
    pub base_type: JvmtiParamTypes,
    pub null_ok: bool,
}


impl<'a> From<jvmtiParamInfo> for JParamInfo<'a> {
    fn from(info: jvmtiParamInfo) -> Self {
        JParamInfo {
            internal: info,
            lifetime: PhantomData,

            name: stringify(info.name),
            kind: info.kind.into(),
            base_type: info.base_type.into(),
            null_ok: to_bool(info.null_ok),
        }
    }
}

impl<'a> ::std::ops::Deref for JParamInfo<'a> {
    type Target = jvmtiParamInfo;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}