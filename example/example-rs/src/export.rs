use std::os::raw::c_char;

use jvmti::{sys, errors::*, objects::*, event::*};

use std::panic;
use log::{debug, warn, error};
use std::ffi::c_void;
use jni::JNIEnv;
use jni_sys::jint;
use std::string::String;
use std::sync::Once;
use jvmti::objects::JCompiledMethodLoadRecord;

fn from_platform(_input: *const c_char) -> Result<Option<String>> {
    Ok(None)
}

#[allow(non_snake_case)]
#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn Agent_OnLoad(vm: *mut sys::JavaVM, options: *const c_char, reserved: *const c_void) -> sys::jint {
    debug!("Agent 'on load'");
    let log_init_result = panic::catch_unwind(|| {
        log4rs::init_file("log.yaml", Default::default()).unwrap()
    });
    match log_init_result {
        Ok(r) => {}
        Err(e) => {
            println!("Failed to initialize logger: {:?}", e);
            return -1;
        }
    }

    let result = panic::catch_unwind(|| {
        debug!("Agent 'on load'");
        // Options is a platform string, not modified utf-8 string, see https://bugs.openjdk.java.net/browse/JDK-5049313
        let options_string = from_platform(options);
        return match options_string {
            Ok(s) => {
                let javaVM = java_vm!(vm);
                jvmti::agent_on_load(&javaVM, &s, initialize)
            }
            Err(e) => {
                error!("Failed to process options string {}", e);
                -1
            }
        };
    });
    match result {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to load agent: {:?}", e);
            -1
        }
    }
}

fn method_entry(event: MethodEntryEvent) {
    debug!("method_entry => {:?}", event.method);
}

fn class_prepare(event: ClassPrepareEvent) {
    debug!("class_prepare => {:?}", event.klass);
}

fn vm_start(event: VmStartEvent) {
    debug!("vm_start");
    jvmti_catch!(event, get_version_number, {|e:&jint| {
        debug!("vm_start  => get_version_number(): {}", e)
    }});
}

fn dynamic_code_generated(event: DynamicCodeGeneratedEvent) {
    debug!("dynamic_code_generated => {}", event.name)
}

fn compiled_method_load(event: CompiledMethodLoadEvent) {
    debug!("compiled_method_load => {:?}", event.method);

    if event.compiled_records.is_some() {
        match event.compiled_records {
            &Some(ref infos) => {
                debug!("compiled_method_load => compiled_records: {}", infos.len());
                for info in infos.iter() {
                    match info {
                        &JCompiledMethodLoadRecord::Inline { ref stack_infos } => {
                            debug!("compiled_method_load => compiled_records.stack_infos: {}", stack_infos.len());
                        }
                        _ => {}
                    }
                }
            }
            &None => {}
        }
    }
}

fn vm_object_alloc(event: VmObjectAllocEvent) {
    debug!("vm_object_alloc => {}", event.size)
}

static ONCE: Once = Once::new();

fn class_load(event: ClassLoadEvent) {
    ONCE.call_once(|| {
        jvmti_catch!(event, get_system_properties, {|items:&Vec<JvmtiString>| {
            debug!("class_load  => get_system_properties(): {}", items.len());
            for item in items.iter() {
                let str: String = item.into();
                debug!("class_load  => get_system_properties(): {}", str);
            }
        }});

        jvmti_catch!(event, get_system_property, {|e:&JvmtiString| {
            let str: String = e.into();
            debug!("class_load  => get_system_property(): {}", str);
        }}, "java.home");

        jvmti_catch!(event, get_jni, {|e:&JNIEnv| {
            match e.get_version() {
                Ok(v) => debug!("class_load  => get_jni().get_version(): {:?}", v),
                Err(e) => error!("class_load  => get_jni().get_version(): {}", e)
            }
        }});
    })
}

fn exception(event: ExceptionEvent) {
    debug!("exception => {:?}, {:?}, {}", event.thread, event.method, event.location)
}

fn exception_catch(event: ExceptionCatchEvent) {
    debug!("exception_catch => {:?}, {:?}, {}, {:?}", event.thread, event.method, event.location, event.exception)
}

fn initialize(event_manager: &mut JEventManager) {
    event_manager.get_capabilities().can_generate_all_class_hook_events();
    event_manager.get_capabilities().can_tag_objects();
    event_manager.get_capabilities().can_get_source_file_name();
    event_manager.get_capabilities().can_get_line_numbers();

    // event_manager.compiled_method_load_enabled(None);
    // event_manager.dynamic_code_generated_enabled(None);
    // event_manager.class_prepare_enabled(None);
    // event_manager.method_entry_enabled(None);
    event_manager.vm_object_alloc_enabled(None);
    event_manager.vm_start_enabled();
    event_manager.class_load_enabled(None);
    event_manager.exception_enabled(None);
    event_manager.exception_catch_enabled(None);

    EventAdjuster::on_class_prepare(Some(class_prepare));
    EventAdjuster::on_vm_start(Some(vm_start));
    EventAdjuster::on_dynamic_code_generated(Some(dynamic_code_generated));
    EventAdjuster::on_method_entry(Some(method_entry));
    EventAdjuster::on_compiled_method_load(Some(compiled_method_load));
    EventAdjuster::on_class_load(Some(class_load));
    EventAdjuster::on_vm_object_alloc(Some(vm_object_alloc));
    EventAdjuster::on_exception(Some(exception));
    EventAdjuster::on_exception_catch(Some(exception_catch));
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Agent_OnUnload(vm: *mut sys::JavaVM) {
    debug!("Agent 'on unload'");
    let result = panic::catch_unwind(|| {
        let javaVM = java_vm!(vm);
        jvmti::agent_on_unload(&javaVM)
    });
    match result {
        Ok(()) => (),
        Err(e) => {
            warn!("Failed to unload agent: {:?}", e);
        }
    }
}