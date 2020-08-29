use jni::JavaVM;
use log::debug;

use crate::{*, errors::*};

pub fn do_on_load<'a>(vm: &JavaVM, options: *const c_char, initialize: fn(*const c_char, &mut JEventManager)) -> Result<()> {
    let jvmti = vm.get_jvmti_env()?;
    debug!("Environment obtained");

    let mut capabilities = jvmti.get_capabilities()?;
    let mut event_manger = JEventManager::new(&mut capabilities);

    initialize(options, &mut event_manger);

    let ref env = jvmti;
    event_manger.apply(env);
    debug!("Events enabled for the environment");
    Ok(())
}