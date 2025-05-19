#![doc = "Sidecar module for class [`TlsOptions`][crate::classes::TlsOptions].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TLSOptions` enums](https://docs.godotengine.org/en/stable/classes/class_tlsoptions.html#enumerations).\n\n"]
use godot_ffi as sys;
use crate::builtin::*;
use crate::meta::{
    AsArg, AsObjectArg, ClassName, CowArg, ObjectArg, ObjectCow, PtrcallSignatureTuple, RefArg, VarcallSignatureTuple
};
use crate::classes::native::*;
use crate::classes::Object;
use crate::obj::Gd;
use crate::sys::GodotFfi as _;
use crate::classes::notify::*;
use std::ffi::c_void;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `TLSOptions.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`tls_options`][crate::classes::tls_options]: sidecar module with related enum/flag types\n* [`ITlsOptions`][crate::classes::ITlsOptions]: virtual methods\n\n\nSee also [Godot docs for `TLSOptions`](https://docs.godotengine.org/en/stable/classes/class_tlsoptions.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<TlsOptions>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TlsOptions {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`TlsOptions`][crate::classes::TlsOptions].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TLSOptions` methods](https://docs.godotengine.org/en/stable/classes/class_tlsoptions.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITlsOptions: crate::obj::GodotClass < Base = TlsOptions > + crate::private::You_forgot_the_attribute__godot_api {
        #[doc(hidden)]
        fn register_class(builder: &mut crate::builder::ClassBuilder < Self >) {
            unimplemented !()
        }
        #[doc = r" Godot constructor, accepting an injected `base` object."]
        #[doc = r""]
        #[doc = r" `base` refers to the base instance of the class, which can either be stored in a `Base<T>` field or discarded."]
        #[doc = r" This method returns a fully-constructed instance, which will then be moved into a [`Gd<T>`][crate::obj::Gd] pointer."]
        #[doc = r""]
        #[doc = r" If the class has a `#[class(init)]` attribute, this method will be auto-generated and must not be overridden."]
        fn init(base: crate::obj::Base < Self::Base >) -> Self {
            unimplemented !()
        }
        #[doc = r" String representation of the Godot instance."]
        #[doc = r""]
        #[doc = r" Override this method to define how the instance is represented as a string."]
        #[doc = r" Used by `impl Display for Gd<T>`, as well as `str()` and `print()` in GDScript."]
        fn to_string(&self) -> crate::builtin::GString {
            unimplemented !()
        }
        #[doc = r" Called when the object receives a Godot notification."]
        #[doc = r""]
        #[doc = r" The type of notification can be identified through `what`. The enum is designed to hold all possible `NOTIFICATION_*`"]
        #[doc = r" constants that the current class can handle. However, this is not validated in Godot, so an enum variant `Unknown` exists"]
        #[doc = r" to represent integers out of known constants (mistakes or future additions)."]
        #[doc = r""]
        #[doc = r" This method is named `_notification` in Godot, but `on_notification` in Rust. To _send_ notifications, use the"]
        #[doc = r" [`Object::notify`][crate::classes::Object::notify] method."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_notification`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-method-notification)."]
        #[doc = r" * [Notifications tutorial](https://docs.godotengine.org/en/stable/tutorials/best_practices/godot_notifications.html)."]
        fn on_notification(&mut self, what: ObjectNotification) {
            unimplemented !()
        }
        #[doc = r" Called whenever [`get()`](crate::classes::Object::get) is called or Godot gets the value of a property."]
        #[doc = r""]
        #[doc = r" Should return the given `property`'s value as `Some(value)`, or `None` if the property should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-get)."]
        fn get_property(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`set()`](crate::classes::Object::set) is called or Godot sets the value of a property."]
        #[doc = r""]
        #[doc = r" Should set `property` to the given `value` and return `true`, or return `false` to indicate the `property`"]
        #[doc = r" should be handled normally."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_set`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-set)."]
        fn set_property(&mut self, property: StringName, value: Variant) -> bool {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot [`get_property_list()`](crate::classes::Object::get_property_list) is called, the returned vector here is"]
        #[doc = r" appended to the existing list of properties."]
        #[doc = r""]
        #[doc = r" This should mainly be used for advanced purposes, such as dynamically updating the property list in the editor."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_get_property_list`](https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-get-property-list)"]
        #[cfg(since_api = "4.3")]
        fn get_property_list(&mut self) -> Vec < crate::meta::PropertyInfo > {
            unimplemented !()
        }
        #[doc = r" Called whenever Godot retrieves value of property. Allows to customize existing properties."]
        #[doc = r" Every property info goes through this method, except properties **added** with `get_property_list()`."]
        #[doc = r""]
        #[doc = r" Exposed `property` here is a shared mutable reference obtained (and returned to) from Godot."]
        #[doc = r""]
        #[doc = r" See also in the Godot docs:"]
        #[doc = r" * [`Object::_validate_property`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-private-method-validate-property)"]
        #[cfg(since_api = "4.2")]
        fn validate_property(&self, property: &mut crate::meta::PropertyInfo) {
            unimplemented !()
        }
        #[doc = r" Called by Godot to tell if a property has a custom revert or not."]
        #[doc = r""]
        #[doc = r" Return `None` for no custom revert, and return `Some(value)` to specify the custom revert."]
        #[doc = r""]
        #[doc = r" This is a combination of Godot's [`Object::_property_get_revert`] and [`Object::_property_can_revert`]. This means that this"]
        #[doc = r" function will usually be called twice by Godot to find the revert."]
        #[doc = r""]
        #[doc = r" Note that this should be a _pure_ function. That is, it should always return the same value for a property as long as `self`"]
        #[doc = r" remains unchanged. Otherwise, this may lead to unexpected (safe) behavior."]
        #[doc = r""]
        #[doc = r" [`Object::_property_get_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-get-revert"]
        #[doc = r" [`Object::_property_can_revert`]: https://docs.godotengine.org/en/latest/classes/class_object.html#class-object-private-method-property-can-revert"]
        #[doc(alias = "property_can_revert")]
        fn property_get_revert(&self, property: StringName) -> Option < Variant > {
            unimplemented !()
        }
    }
    impl TlsOptions {
        pub(crate) fn client_full(trusted_chain: ObjectArg < crate::classes::X509Certificate >, common_name_override: CowArg < GString >,) -> Option < Gd < crate::classes::TlsOptions > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::TlsOptions > >, ObjectArg < crate::classes::X509Certificate >, CowArg < 'a0, GString >);
            let args = (trusted_chain, common_name_override,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8221usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TlsOptions", "client", std::ptr::null_mut(), None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::client_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn client() -> Option < Gd < crate::classes::TlsOptions > > {
            Self::client_ex() . done()
        }
        #[inline]
        pub fn client_ex < 'a > () -> ExClient < 'a > {
            ExClient::new()
        }
        pub(crate) fn client_unsafe_full(trusted_chain: ObjectArg < crate::classes::X509Certificate >,) -> Option < Gd < crate::classes::TlsOptions > > {
            type CallSig = (Option < Gd < crate::classes::TlsOptions > >, ObjectArg < crate::classes::X509Certificate >);
            let args = (trusted_chain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8222usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TlsOptions", "client_unsafe", std::ptr::null_mut(), None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::client_unsafe_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn client_unsafe() -> Option < Gd < crate::classes::TlsOptions > > {
            Self::client_unsafe_ex() . done()
        }
        #[inline]
        pub fn client_unsafe_ex < 'a > () -> ExClientUnsafe < 'a > {
            ExClientUnsafe::new()
        }
        pub fn server(key: impl AsObjectArg < crate::classes::CryptoKey >, certificate: impl AsObjectArg < crate::classes::X509Certificate >,) -> Option < Gd < crate::classes::TlsOptions > > {
            type CallSig = (Option < Gd < crate::classes::TlsOptions > >, ObjectArg < crate::classes::CryptoKey >, ObjectArg < crate::classes::X509Certificate >);
            let args = (key.as_object_arg(), certificate.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8223usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TlsOptions", "server", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn is_server(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8224usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TlsOptions", "is_server", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_unsafe_client(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8225usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TlsOptions", "is_unsafe_client", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_common_name_override(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8226usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TlsOptions", "get_common_name_override", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_trusted_ca_chain(&self,) -> Option < Gd < crate::classes::X509Certificate > > {
            type CallSig = (Option < Gd < crate::classes::X509Certificate > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8227usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TlsOptions", "get_trusted_ca_chain", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_private_key(&self,) -> Option < Gd < crate::classes::CryptoKey > > {
            type CallSig = (Option < Gd < crate::classes::CryptoKey > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8228usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TlsOptions", "get_private_key", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_own_certificate(&self,) -> Option < Gd < crate::classes::X509Certificate > > {
            type CallSig = (Option < Gd < crate::classes::X509Certificate > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8229usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "TlsOptions", "get_own_certificate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        fn __checked_id(&self) -> Option < crate::obj::InstanceId > {
            let rtti = unsafe {
                self.rtti.as_ref() . unwrap_unchecked()
            };
            let instance_id = rtti.check_type::< Self > ();
            Some(instance_id)
        }
        #[doc(hidden)]
        pub fn __object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
    }
    impl crate::obj::GodotClass for TlsOptions {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"TLSOptions"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TlsOptions {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TlsOptions {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TlsOptions {
        
    }
    impl std::ops::Deref for TlsOptions {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TlsOptions {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`TlsOptions`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_TlsOptions {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::TlsOptions > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`TlsOptions::client_ex`][super::TlsOptions::client_ex]."]
#[must_use]
pub struct ExClient < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, trusted_chain: ObjectCow < crate::classes::X509Certificate >, common_name_override: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClient < 'a > {
    fn new() -> Self {
        let trusted_chain = Gd::null_arg();
        let common_name_override = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, trusted_chain: trusted_chain.consume_arg(), common_name_override: CowArg::Owned(common_name_override),
        }
    }
    #[inline]
    pub fn trusted_chain(self, trusted_chain: impl AsObjectArg < crate::classes::X509Certificate >) -> Self {
        Self {
            trusted_chain: trusted_chain.consume_arg(), .. self
        }
    }
    #[inline]
    pub fn common_name_override(self, common_name_override: impl AsArg < GString > + 'a) -> Self {
        Self {
            common_name_override: common_name_override.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TlsOptions > > {
        let Self {
            _phantom, trusted_chain, common_name_override,
        }
        = self;
        re_export::TlsOptions::client_full(trusted_chain.cow_as_object_arg(), common_name_override,)
    }
}
#[doc = "Default-param extender for [`TlsOptions::client_unsafe_ex`][super::TlsOptions::client_unsafe_ex]."]
#[must_use]
pub struct ExClientUnsafe < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, trusted_chain: ObjectCow < crate::classes::X509Certificate >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClientUnsafe < 'a > {
    fn new() -> Self {
        let trusted_chain = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, trusted_chain: trusted_chain.consume_arg(),
        }
    }
    #[inline]
    pub fn trusted_chain(self, trusted_chain: impl AsObjectArg < crate::classes::X509Certificate >) -> Self {
        Self {
            trusted_chain: trusted_chain.consume_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TlsOptions > > {
        let Self {
            _phantom, trusted_chain,
        }
        = self;
        re_export::TlsOptions::client_unsafe_full(trusted_chain.cow_as_object_arg(),)
    }
}