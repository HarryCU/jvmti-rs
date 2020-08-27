use crate::{errors::*, *};
use log::debug;
use jni::JavaVM;

pub fn do_on_load<'a>(vm: &JavaVM, _options: &Option<String>, initialize: fn(&mut JEventManager)) -> Result<()> {
    let jvmti = vm.get_jvmti_env()?;
    debug!("Environment obtained");

    let mut capabilities = jvmti.get_capabilities()?;
    let mut event_manger = JEventManager::new(&mut capabilities);

    initialize(&mut event_manger);

    let ref env = jvmti;
    event_manger.apply(env);
    debug!("Events enabled for the environment");
    Ok(())
}