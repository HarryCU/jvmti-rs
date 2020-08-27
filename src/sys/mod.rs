#![allow(non_snake_case, non_camel_case_types)]

mod cmlr;
mod utils;
mod r#type;
mod jvmtienv;
mod constant;
mod capabilities;

pub use jni::sys::*;

pub use cmlr::*;
pub use utils::*;
pub use r#type::*;
pub use jvmtienv::*;
pub use constant::*;
pub use capabilities::*;

use std::os::raw::c_char;
use std::os::raw::c_void;

pub const JVMTI_VERSION_1: i32 = 0x30010000;
pub const JVMTI_VERSION_1_0: i32 = 0x30010000;
pub const JVMTI_VERSION_1_1: i32 = 0x30010100;
pub const JVMTI_VERSION_1_2: i32 = 0x30010200;

pub const JVMTI_VERSION: i32 = 0x30000000 + (1 * 0x10000) + (2 * 0x100) + 1;  /* version: 1.2.1 */

extern "C" {
    pub fn Agent_OnLoad(
        vm: *mut JavaVM,
        options: *mut c_char,
        reserved: *mut c_void,
    ) -> jint;
    pub fn Agent_OnAttach(
        vm: *mut JavaVM,
        options: *mut c_char,
        reserved: *mut c_void,
    ) -> jint;
    pub fn Agent_OnUnload(vm: *mut JavaVM);
}
