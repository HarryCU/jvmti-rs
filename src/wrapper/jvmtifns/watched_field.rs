use crate::{objects::*, errors::*, JVMTIEnv, Desc};
use jni::strings::JNIString;

impl<'a> JVMTIEnv<'a> {
    pub fn set_field_access_watch(&self, klass: &JClass, field: &JFieldID) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), SetFieldAccessWatch,
            klass.into_inner(),
            field.into_inner()
        )
    }

    pub fn clear_field_access_watch(&self, klass: &JClass, field: &JFieldID) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), ClearFieldAccessWatch,
            klass.into_inner(),
            field.into_inner()
        )
    }

    pub fn set_field_modification_watch(&self, klass: &JClass, field: &JFieldID) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), SetFieldModificationWatch,
            klass.into_inner(),
            field.into_inner()
        )
    }

    pub fn clear_field_modification_watch(&self, klass: &JClass, field: &JFieldID) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), ClearFieldModificationWatch,
            klass.into_inner(),
            field.into_inner()
        )
    }

    pub fn set_field_access_watch_s<K, F, V>(&self, class: K, name: F, sig: V) -> Result<()>
        where
            K: Into<JNIString>,
            F: Into<JNIString>,
            V: Into<JNIString> {
        let class_name = class.into();

        let klass: JClass = class_name.lookup(self)?;
        let field: JFieldID = (klass, name, sig).lookup(self)?;

        jvmti_call!(self.jvmti_raw(), SetFieldAccessWatch,
            klass.into_inner(),
            field.into_inner()
        )
    }

    pub fn clear_field_access_watch_s<K, F, V>(&self, class: K, name: F, sig: V) -> Result<()>
        where
            K: Into<JNIString>,
            F: Into<JNIString>,
            V: Into<JNIString> {
        let class_name = class.into();

        let klass: JClass = class_name.lookup(self)?;
        let field: JFieldID = (klass, name, sig).lookup(self)?;

        jvmti_call!(self.jvmti_raw(), ClearFieldAccessWatch,
            klass.into_inner(),
            field.into_inner()
        )
    }

    pub fn set_field_modification_watch_s<K, F, V>(&self, class: K, name: F, sig: V) -> Result<()>
        where
            K: Into<JNIString>,
            F: Into<JNIString>,
            V: Into<JNIString> {
        let class_name = class.into();

        let klass: JClass = class_name.lookup(self)?;
        let field: JFieldID = (klass, name, sig).lookup(self)?;

        jvmti_call!(self.jvmti_raw(), SetFieldModificationWatch,
            klass.into_inner(),
            field.into_inner()
        )
    }

    pub fn clear_field_modification_watch_s<K, F, V>(&self, class: K, name: F, sig: V) -> Result<()>
        where
            K: Into<JNIString>,
            F: Into<JNIString>,
            V: Into<JNIString> {
        let class_name = class.into();
        let klass: JClass = class_name.lookup(self)?;
        let field: JFieldID = (klass, name, sig).lookup(self)?;

        jvmti_call!(self.jvmti_raw(), ClearFieldModificationWatch,
            klass.into_inner(),
            field.into_inner()
        )
    }
}
