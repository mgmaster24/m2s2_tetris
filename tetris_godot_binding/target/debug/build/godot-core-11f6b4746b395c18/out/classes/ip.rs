#![doc = "Sidecar module for class [`Ip`][crate::classes::Ip].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `IP` enums](https://docs.godotengine.org/en/stable/classes/class_ip.html#enumerations).\n\n"]
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
    #[doc = "Godot class `IP.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`ip`][crate::classes::ip]: sidecar module with related enum/flag types\n* [`IIp`][crate::classes::IIp]: virtual methods\n\n\nSee also [Godot docs for `IP`](https://docs.godotengine.org/en/stable/classes/class_ip.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Ip::singleton()`][Ip::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Ip {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`Ip`][crate::classes::Ip].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `IP` methods](https://docs.godotengine.org/en/stable/classes/class_ip.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IIp: crate::obj::GodotClass < Base = Ip > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Ip {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"IP");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub(crate) fn resolve_hostname_full(&mut self, host: CowArg < GString >, ip_type: crate::classes::ip::Type,) -> GString {
            type CallSig < 'a0, > = (GString, CowArg < 'a0, GString >, crate::classes::ip::Type);
            let args = (host, ip_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4102usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Ip", "resolve_hostname", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::resolve_hostname_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn resolve_hostname(&mut self, host: impl AsArg < GString >,) -> GString {
            self.resolve_hostname_ex(host,) . done()
        }
        #[inline]
        pub fn resolve_hostname_ex < 'a > (&'a mut self, host: impl AsArg < GString > + 'a,) -> ExResolveHostname < 'a > {
            ExResolveHostname::new(self, host,)
        }
        pub(crate) fn resolve_hostname_addresses_full(&mut self, host: CowArg < GString >, ip_type: crate::classes::ip::Type,) -> PackedStringArray {
            type CallSig < 'a0, > = (PackedStringArray, CowArg < 'a0, GString >, crate::classes::ip::Type);
            let args = (host, ip_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4103usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Ip", "resolve_hostname_addresses", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::resolve_hostname_addresses_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn resolve_hostname_addresses(&mut self, host: impl AsArg < GString >,) -> PackedStringArray {
            self.resolve_hostname_addresses_ex(host,) . done()
        }
        #[inline]
        pub fn resolve_hostname_addresses_ex < 'a > (&'a mut self, host: impl AsArg < GString > + 'a,) -> ExResolveHostnameAddresses < 'a > {
            ExResolveHostnameAddresses::new(self, host,)
        }
        pub(crate) fn resolve_hostname_queue_item_full(&mut self, host: CowArg < GString >, ip_type: crate::classes::ip::Type,) -> i32 {
            type CallSig < 'a0, > = (i32, CowArg < 'a0, GString >, crate::classes::ip::Type);
            let args = (host, ip_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4104usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Ip", "resolve_hostname_queue_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::resolve_hostname_queue_item_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn resolve_hostname_queue_item(&mut self, host: impl AsArg < GString >,) -> i32 {
            self.resolve_hostname_queue_item_ex(host,) . done()
        }
        #[inline]
        pub fn resolve_hostname_queue_item_ex < 'a > (&'a mut self, host: impl AsArg < GString > + 'a,) -> ExResolveHostnameQueueItem < 'a > {
            ExResolveHostnameQueueItem::new(self, host,)
        }
        pub fn get_resolve_item_status(&self, id: i32,) -> crate::classes::ip::ResolverStatus {
            type CallSig = (crate::classes::ip::ResolverStatus, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4105usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Ip", "get_resolve_item_status", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_resolve_item_address(&self, id: i32,) -> GString {
            type CallSig = (GString, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4106usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Ip", "get_resolve_item_address", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_resolve_item_addresses(&self, id: i32,) -> VariantArray {
            type CallSig = (VariantArray, i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4107usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Ip", "get_resolve_item_addresses", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn erase_resolve_item(&mut self, id: i32,) {
            type CallSig = ((), i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4108usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Ip", "erase_resolve_item", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_local_addresses(&self,) -> PackedStringArray {
            type CallSig = (PackedStringArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4109usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Ip", "get_local_addresses", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_local_interfaces(&self,) -> Array < Dictionary > {
            type CallSig = (Array < Dictionary >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4110usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Ip", "get_local_interfaces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn clear_cache_full(&mut self, hostname: CowArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (hostname,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4111usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "Ip", "clear_cache", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::clear_cache_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn clear_cache(&mut self,) {
            self.clear_cache_ex() . done()
        }
        #[inline]
        pub fn clear_cache_ex < 'a > (&'a mut self,) -> ExClearCache < 'a > {
            ExClearCache::new(self,)
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
        pub const RESOLVER_MAX_QUERIES: i32 = 256i32;
        pub const RESOLVER_INVALID_ID: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for Ip {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"IP"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Ip {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Ip {
        
    }
    impl std::ops::Deref for Ip {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Ip {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Ip`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_Ip {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Ip > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Ip::resolve_hostname_ex`][super::Ip::resolve_hostname_ex]."]
#[must_use]
pub struct ExResolveHostname < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Ip, host: CowArg < 'a, GString >, ip_type: crate::classes::ip::Type,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExResolveHostname < 'a > {
    fn new(surround_object: &'a mut re_export::Ip, host: impl AsArg < GString > + 'a,) -> Self {
        let ip_type = crate::obj::EngineEnum::from_ord(3);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, host: host.into_arg(), ip_type: ip_type,
        }
    }
    #[inline]
    pub fn ip_type(self, ip_type: crate::classes::ip::Type) -> Self {
        Self {
            ip_type: ip_type, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, host, ip_type,
        }
        = self;
        re_export::Ip::resolve_hostname_full(surround_object, host, ip_type,)
    }
}
#[doc = "Default-param extender for [`Ip::resolve_hostname_addresses_ex`][super::Ip::resolve_hostname_addresses_ex]."]
#[must_use]
pub struct ExResolveHostnameAddresses < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Ip, host: CowArg < 'a, GString >, ip_type: crate::classes::ip::Type,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExResolveHostnameAddresses < 'a > {
    fn new(surround_object: &'a mut re_export::Ip, host: impl AsArg < GString > + 'a,) -> Self {
        let ip_type = crate::obj::EngineEnum::from_ord(3);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, host: host.into_arg(), ip_type: ip_type,
        }
    }
    #[inline]
    pub fn ip_type(self, ip_type: crate::classes::ip::Type) -> Self {
        Self {
            ip_type: ip_type, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedStringArray {
        let Self {
            _phantom, surround_object, host, ip_type,
        }
        = self;
        re_export::Ip::resolve_hostname_addresses_full(surround_object, host, ip_type,)
    }
}
#[doc = "Default-param extender for [`Ip::resolve_hostname_queue_item_ex`][super::Ip::resolve_hostname_queue_item_ex]."]
#[must_use]
pub struct ExResolveHostnameQueueItem < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Ip, host: CowArg < 'a, GString >, ip_type: crate::classes::ip::Type,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExResolveHostnameQueueItem < 'a > {
    fn new(surround_object: &'a mut re_export::Ip, host: impl AsArg < GString > + 'a,) -> Self {
        let ip_type = crate::obj::EngineEnum::from_ord(3);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, host: host.into_arg(), ip_type: ip_type,
        }
    }
    #[inline]
    pub fn ip_type(self, ip_type: crate::classes::ip::Type) -> Self {
        Self {
            ip_type: ip_type, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, host, ip_type,
        }
        = self;
        re_export::Ip::resolve_hostname_queue_item_full(surround_object, host, ip_type,)
    }
}
#[doc = "Default-param extender for [`Ip::clear_cache_ex`][super::Ip::clear_cache_ex]."]
#[must_use]
pub struct ExClearCache < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::Ip, hostname: CowArg < 'a, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExClearCache < 'a > {
    fn new(surround_object: &'a mut re_export::Ip,) -> Self {
        let hostname = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, hostname: CowArg::Owned(hostname),
        }
    }
    #[inline]
    pub fn hostname(self, hostname: impl AsArg < GString > + 'a) -> Self {
        Self {
            hostname: hostname.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, hostname,
        }
        = self;
        re_export::Ip::clear_cache_full(surround_object, hostname,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ResolverStatus {
    ord: i32
}
impl ResolverStatus {
    #[doc(alias = "RESOLVER_STATUS_NONE")]
    #[doc = "Godot enumerator name: `RESOLVER_STATUS_NONE`"]
    pub const NONE: ResolverStatus = ResolverStatus {
        ord: 0i32
    };
    #[doc(alias = "RESOLVER_STATUS_WAITING")]
    #[doc = "Godot enumerator name: `RESOLVER_STATUS_WAITING`"]
    pub const WAITING: ResolverStatus = ResolverStatus {
        ord: 1i32
    };
    #[doc(alias = "RESOLVER_STATUS_DONE")]
    #[doc = "Godot enumerator name: `RESOLVER_STATUS_DONE`"]
    pub const DONE: ResolverStatus = ResolverStatus {
        ord: 2i32
    };
    #[doc(alias = "RESOLVER_STATUS_ERROR")]
    #[doc = "Godot enumerator name: `RESOLVER_STATUS_ERROR`"]
    pub const ERROR: ResolverStatus = ResolverStatus {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ResolverStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ResolverStatus") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ResolverStatus {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NONE => "NONE", Self::WAITING => "WAITING", Self::DONE => "DONE", Self::ERROR => "ERROR", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NONE => "RESOLVER_STATUS_NONE", Self::WAITING => "RESOLVER_STATUS_WAITING", Self::DONE => "RESOLVER_STATUS_DONE", Self::ERROR => "RESOLVER_STATUS_ERROR", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for ResolverStatus {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ResolverStatus {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ResolverStatus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Type {
    ord: i32
}
impl Type {
    #[doc(alias = "TYPE_NONE")]
    #[doc = "Godot enumerator name: `TYPE_NONE`"]
    pub const NONE: Type = Type {
        ord: 0i32
    };
    #[doc(alias = "TYPE_IPV4")]
    #[doc = "Godot enumerator name: `TYPE_IPV4`"]
    pub const IPV4: Type = Type {
        ord: 1i32
    };
    #[doc(alias = "TYPE_IPV6")]
    #[doc = "Godot enumerator name: `TYPE_IPV6`"]
    pub const IPV6: Type = Type {
        ord: 2i32
    };
    #[doc(alias = "TYPE_ANY")]
    #[doc = "Godot enumerator name: `TYPE_ANY`"]
    pub const ANY: Type = Type {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Type") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Type {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self {
                ord
            }), _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
    #[inline]
    fn as_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NONE => "NONE", Self::IPV4 => "IPV4", Self::IPV6 => "IPV6", Self::ANY => "ANY", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::NONE => "TYPE_NONE", Self::IPV4 => "TYPE_IPV4", Self::IPV6 => "TYPE_IPV6", Self::ANY => "TYPE_ANY", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for Type {
    type Via = i32;
    
}
impl crate::meta::ToGodot for Type {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Type {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}