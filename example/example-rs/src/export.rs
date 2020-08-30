use std::ffi::c_void;
use std::os::raw::c_char;
use std::panic;
use std::string::String;
use std::sync::Once;

use jni::JNIEnv;
use jni_sys::jint;
use log::{debug, error, warn};

use jvmti::objects::{JCompiledMethodLoadRecord, JStaticMethodID};
use jvmti::{event::*, sys, *};

#[allow(non_snake_case)]
#[allow(unused_variables)]
#[no_mangle]
pub extern "C" fn Agent_OnLoad(
    vm: *mut sys::JavaVM,
    options: *const c_char,
    reserved: *const c_void,
) -> sys::jint {
    debug!("Agent 'on load'");
    let log_init_result =
        panic::catch_unwind(|| log4rs::init_file("log.yaml", Default::default()).unwrap());
    match log_init_result {
        Ok(r) => {}
        Err(e) => {
            println!("Failed to initialize logger: {:?}", e);
            return -1;
        }
    }

    let result = panic::catch_unwind(|| {
        let javaVM = java_vm!(vm);
        jvmti::agent_on_load(&javaVM, options, initialize)
    });
    match result {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to load agent: {:?}", e);
            -1
        }
    }
}

fn initialize(_options: *const c_char, event_manager: &mut JEventManager) {
    event_manager
        .get_capabilities()
        .can_generate_all_class_hook_events();
    event_manager.get_capabilities().can_tag_objects();
    event_manager.get_capabilities().can_get_source_file_name();
    event_manager.get_capabilities().can_get_line_numbers();

    // event_manager.compiled_method_load_enabled(None);
    // event_manager.dynamic_code_generated_enabled(None);
    // event_manager.class_prepare_enabled(None);
    // event_manager.vm_object_alloc_enabled(None);
    event_manager.method_entry_enabled(None);
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

fn method_entry(event: MethodEntryEvent) {
    let method_name = event.jvmti.get_method_name(event.method).unwrap();

    if method_name.name == "debug" {
        debug!("method_entry => {:?}", method_name);

        let result = panic::catch_unwind(|| {
            let klass = event.jvmti.get_class("HelloWorld").unwrap();
            debug!("get_class => {:?}", klass);

            jvmti_catch!(
                event.jvmti,
                get_static_method_id,
                |e: &JStaticMethodID| {
                    debug!("get_static_method_id => {:?}", e.into_inner());
                },
                klass,
                "debug",
                "(Ljava/lang/Integer;)V"
            );

            let arg_size = event
                .jvmti
                .get_arguments_size_s("HelloWorld", "debug", "(Ljava/lang/Integer;)V")
                .unwrap();
            debug!("get_arguments_size_s => {}", arg_size);

            let all_threads = event.jvmti.get_all_threads().unwrap();
            debug!("get_all_threads => {}", all_threads.len());

            let _ = event.jvmti.get_jni().unwrap();
            debug!("get_jni => complete.");
        });
        match result {
            Ok(_) => (),
            Err(e) => {
                error!("panic: {:?}", e);
            }
        }
    }
}

fn class_prepare(event: ClassPrepareEvent) {
    debug!("class_prepare => {:?}", event.klass);
}

fn vm_start(event: VmStartEvent) {
    debug!("vm_start");
    jvmti_catch!(event.jvmti, get_version_number, |e: &jint| {
        debug!("vm_start  => get_version_number(): {}", e)
    });
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
                            debug!(
                                "compiled_method_load => compiled_records.stack_infos: {}",
                                stack_infos.len()
                            );
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
        jvmti_catch!(event.jvmti, get_system_properties, |items: &Vec<String>| {
            debug!("class_load  => get_system_properties(): {}", items.len());
            for item in items.iter() {
                debug!("class_load  => get_system_properties(): {}", item);
            }
        });

        jvmti_catch!(
            event.jvmti,
            get_system_property,
            |e: &String| {
                debug!("class_load  => get_system_property(): {}", e);
            },
            "java.home"
        );

        jvmti_catch!(event.jvmti, get_jni, |e: &JNIEnv| {
            match e.get_version() {
                Ok(v) => debug!("class_load  => get_jni().get_version(): {:?}", v),
                Err(e) => error!("class_load  => get_jni().get_version(): {}", e),
            }
        });
    })
}

fn exception(event: ExceptionEvent) {
    let method_name = event.jvmti.get_method_name(event.method).unwrap();
    debug!(
        "exception => {:?}, {:?}, {:?}, {}",
        event.thread, event.method, method_name, event.location
    )
}

fn exception_catch(event: ExceptionCatchEvent) {
    debug!(
        "exception_catch => {:?}, {:?}, {}, {:?}",
        event.thread, event.method, event.location, event.exception
    )
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
