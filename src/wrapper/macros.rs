
// see https://github.com/jni-rs/jni-rs/blob/master/src/wrapper/macros.rs

macro_rules! jvmti_call {
    ( $jvmtienv:expr, $name:tt $(, $args:expr )* ) => ({
        log::trace!("calling checked jvmti method: {}", stringify!($name));

        let res = unsafe {
            jvmti_method!($jvmtienv, $name)($jvmtienv, $($args),*)
        };

        return jvmti_error_code_to_result(res)
    })
}

macro_rules! jvmti_call_result {
    ( $jvmtienv:expr, $name:tt $(, $args:expr )* ) => ({
        log::trace!("calling checked jvmti method: {}", stringify!($name));

        let res = unsafe {
            jvmti_method!($jvmtienv, $name)($jvmtienv, $($args),*)
        };

        res
    })
}

macro_rules! jvmti_call_number_result {
    ( $jvmtienv:expr, $type:block, $name:tt) => ({
        let def = (||$type)();
        let mut value = def;
        let res = jvmti_call_result!($jvmtienv, $name,
            &mut value
        );
        jvmti_error_code_to_result(res)?;
        value
    });
    ( $jvmtienv:expr, $type:ty, $name:tt) => ({
        let mut value: $type = 0 as $type;
        let res = jvmti_call_result!($jvmtienv, $name,
            &mut value
        );
        jvmti_error_code_to_result(res)?;
        value
    });
    ( $jvmtienv:expr, $type:ty, $name:tt $(, $args:expr )* ) => ({
        let mut value: $type = 0 as $type;
        let res = jvmti_call_result!($jvmtienv, $name,
            $($args),*,
            &mut value
        );
        jvmti_error_code_to_result(res)?;
        value
    });
}

macro_rules! jvmti_call_ptr_result {
    ( $jvmtienv:expr, $type:tt, $name:tt $(, $args:expr )* ) => ({
        let mut value: $type = ptr::null_mut();
        let res = jvmti_call_result!($jvmtienv, $name,
            $($args),*,
            &mut value
        );
        jvmti_error_code_to_result(res)?;
        value
    })
}

macro_rules! non_null {
    ( $obj:expr, $ctx:expr ) => {
        if $obj.is_null() {
            return Err($crate::wrapper::errors::ErrorKind::NullPtr($ctx).into());
        } else {
            $obj
        }
    };
}

macro_rules! jvmti_method {
    ( $jvmtienv:expr, $name:tt ) => {{
        log::trace!("looking up jvmti method {}", stringify!($name));
        let env = $jvmtienv;
        match deref!(deref!(env, "JVMTIEnv"), "*JVMTIEnv").$name {
            Some(method) => {
                log::trace!("found jvmti method");
                method
            }
            None => {
                log::trace!("jvmtienv method not defined, returning error");
                return Err($crate::wrapper::errors::Error::from(
                    $crate::wrapper::errors::ErrorKind::JVMTIEnvMethodNotFound(stringify!($name)),
                ).into());
            }
        }
    }};
}

macro_rules! java_vm_unchecked {
    ( $java_vm:expr, $name:tt $(, $args:expr )* ) => ({
        log::trace!("calling unchecked JavaVM method: {}", stringify!($name));
        java_vm_method!($java_vm, $name)($java_vm, $($args),*)
    })
}


macro_rules! java_vm_method {
    ( $jvmtienv:expr, $name:tt ) => {{
        log::trace!("looking up JavaVM method {}", stringify!($name));
        let env = $jvmtienv;
        match deref!(deref!(env, "JavaVM"), "*JavaVM").$name {
            Some(meth) => {
                log::trace!("found JavaVM method");
                meth
            }
            None => {
                log::trace!("JavaVM method not defined, returning error");
                return Err($crate::wrapper::errors::Error::from(
                    $crate::wrapper::errors::ErrorKind::JavaVMMethodNotFound(stringify!($name)),
                )
                .into());
            }
        }
    }};
}

macro_rules! deref {
    ( $obj:expr, $ctx:expr ) => {
        if $obj.is_null() {
            return Err($crate::wrapper::errors::ErrorKind::NullDeref($ctx).into());
        } else {
            #[allow(unused_unsafe)]
            unsafe {
                *$obj
            }
        }
    };
}


#[macro_export]
macro_rules! jvmti {
    ($jvmti_env:expr) => {
        $crate::JVMTIEnv::from_raw($jvmti_env).unwrap();
    };
}

#[macro_export]
macro_rules! jni {
    ($jni_env:expr) => {
        $crate::jni::JNIEnv::from_raw($jni_env).unwrap();
    };
}

#[macro_export]
macro_rules! java_vm {
    ($vm:expr) => {{
        unsafe {
            $crate::JavaVM::from_raw($vm).unwrap()
        }
    }};
}