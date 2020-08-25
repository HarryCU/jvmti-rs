use crate::sys;

#[derive(Debug)]
pub enum JvmtiJlocationFormat {
    Jvmbci,
    MachinePc,
    Other,
    Unsupported(sys::jvmtiJlocationFormat),
}


impl From<sys::jvmtiJlocationFormat> for JvmtiJlocationFormat {
    fn from(value: sys::jvmtiJlocationFormat) -> Self {
        match value {
            sys::JVMTI_JLOCATION_JVMBCI => JvmtiJlocationFormat::Jvmbci,
            sys::JVMTI_JLOCATION_MACHINEPC => JvmtiJlocationFormat::MachinePc,
            sys::JVMTI_JLOCATION_OTHER => JvmtiJlocationFormat::Other,
            _ => JvmtiJlocationFormat::Unsupported(value)
        }
    }
}

impl From<JvmtiJlocationFormat> for sys::jvmtiJlocationFormat {
    fn from(value: JvmtiJlocationFormat) -> Self {
        match value {
            JvmtiJlocationFormat::Jvmbci => sys::JVMTI_JLOCATION_JVMBCI,
            JvmtiJlocationFormat::MachinePc => sys::JVMTI_JLOCATION_MACHINEPC,
            JvmtiJlocationFormat::Other => sys::JVMTI_JLOCATION_OTHER,
            _ => sys::JVMTI_CONSTANT_UNSUPPORTED,
        }
    }
}