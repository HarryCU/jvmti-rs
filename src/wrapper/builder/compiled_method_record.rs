use std::ffi::c_void;
use crate::wrapper::{JCompiledMethodLoadRecord, MutObjectArrayBuilder, Builder};
use crate::sys::{jvmtiCompiledMethodLoadRecordHeader, JVMTI_CMLR_MAJOR_VERSION, JVMTI_CMLR_MINOR_VERSION,
                 JVMTI_CMLR_DUMMY, JVMTI_CMLR_INLINE_INFO, jvmtiCompiledMethodLoadInlineRecord,
                 PCStackInfo, jvmtiCompiledMethodLoadDummyRecord};
use log::debug;

// see https://github.com/rel-eng/rvmti/blob/master/src/rvmti.rs#L1376
pub fn parse_compiled_method_load_record<'a>(compile_info: *const c_void) -> Option<Vec<JCompiledMethodLoadRecord<'a>>> {
    unsafe {
        if compile_info.is_null() {
            return None;
        }

        let mut result = Vec::new();
        let mut record_ptr = compile_info as *const jvmtiCompiledMethodLoadRecordHeader;
        loop {
            let record = *record_ptr;
            if record.majorinfoversion == JVMTI_CMLR_MAJOR_VERSION && record.minorinfoversion == JVMTI_CMLR_MINOR_VERSION {
                match record.kind {
                    JVMTI_CMLR_DUMMY => {
                        let dummy_record_ptr = record_ptr as *const jvmtiCompiledMethodLoadDummyRecord;
                        if dummy_record_ptr.is_null() {
                            continue;
                        }
                        debug!("jvmtiCompiledMethodLoadDummyRecord: {:?}", dummy_record_ptr);
                        result.push(JCompiledMethodLoadRecord::Dummy);
                    }
                    JVMTI_CMLR_INLINE_INFO => {
                        let inline_record_ptr = record_ptr as *const jvmtiCompiledMethodLoadInlineRecord;
                        if inline_record_ptr.is_null() {
                            continue;
                        }
                        let inline_record = *inline_record_ptr;
                        let builder: MutObjectArrayBuilder<PCStackInfo> = MutObjectArrayBuilder::create(inline_record.numpcs, inline_record.pcinfo);
                        result.push(JCompiledMethodLoadRecord::Inline { stack_infos: builder.build() });
                    }
                    _ => {}
                }
            }
            if record.next.is_null() {
                break;
            } else {
                record_ptr = record.next;
            }
        }
        Some(result)
    }
}