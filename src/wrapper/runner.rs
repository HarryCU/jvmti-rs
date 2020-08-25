use crate::wrapper::*;
use thiserror::Error;
use crate::perf;
use log::{warn, debug};
use jni::JavaVM;

pub fn do_on_load<'a>(vm: &JavaVM, options: &Option<String>, initialize: fn(&mut JCapabilities, &mut JEventManager)) -> Result<()> {
    let mut jvmti = vm.get_jvmti_env()?;
    debug!("Environment obtained");

    let mut capabilities = jvmti.get_capabilities()?;
    let mut event_manger = JEventManager::new();

    initialize(&mut capabilities, &mut event_manger);

    debug!("Event handlers enabled for the environment");

    let _ = jvmti.add_capabilities(&capabilities);
    debug!("Capabilities added to the environment");

    event_manger.apply(&mut jvmti);
    debug!("Events enabled for the environment");
    Ok(())
}