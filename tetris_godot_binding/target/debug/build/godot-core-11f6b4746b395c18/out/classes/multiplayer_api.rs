#![doc = "Sidecar module for class [`MultiplayerApi`][crate::classes::MultiplayerApi].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MultiplayerAPI` enums](https://docs.godotengine.org/en/stable/classes/class_multiplayerapi.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MultiplayerAPI.`\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`multiplayer_api`][crate::classes::multiplayer_api]: sidecar module with related enum/flag types\n* [`IMultiplayerApi`][crate::classes::IMultiplayerApi]: virtual methods\n\n\nSee also [Godot docs for `MultiplayerAPI`](https://docs.godotengine.org/en/stable/classes/class_multiplayerapi.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<MultiplayerApi>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MultiplayerApi {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`MultiplayerApi`][crate::classes::MultiplayerApi].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `MultiplayerAPI` methods](https://docs.godotengine.org/en/stable/classes/class_multiplayerapi.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMultiplayerApi: crate::obj::GodotClass < Base = MultiplayerApi > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl MultiplayerApi {
        pub fn has_multiplayer_peer(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5192usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerApi", "has_multiplayer_peer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_multiplayer_peer(&mut self,) -> Option < Gd < crate::classes::MultiplayerPeer > > {
            type CallSig = (Option < Gd < crate::classes::MultiplayerPeer > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5193usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerApi", "get_multiplayer_peer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_multiplayer_peer(&mut self, peer: impl AsObjectArg < crate::classes::MultiplayerPeer >,) {
            type CallSig = ((), ObjectArg < crate::classes::MultiplayerPeer >);
            let args = (peer.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5194usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerApi", "set_multiplayer_peer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_unique_id(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5195usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerApi", "get_unique_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_server(&mut self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5196usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerApi", "is_server", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_remote_sender_id(&mut self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5197usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerApi", "get_remote_sender_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn poll(&mut self,) -> crate::global::Error {
            type CallSig = (crate::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5198usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerApi", "poll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn rpc_full(&mut self, peer: i32, object: ObjectArg < crate::classes::Object >, method: CowArg < StringName >, arguments: RefArg < VariantArray >,) -> crate::global::Error {
            type CallSig < 'a0, 'a1, > = (crate::global::Error, i32, ObjectArg < crate::classes::Object >, CowArg < 'a0, StringName >, RefArg < 'a1, VariantArray >);
            let args = (peer, object, method, arguments,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5199usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerApi", "rpc", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::rpc_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn rpc(&mut self, peer: i32, object: impl AsObjectArg < crate::classes::Object >, method: impl AsArg < StringName >,) -> crate::global::Error {
            self.rpc_ex(peer, object, method,) . done()
        }
        #[inline]
        pub fn rpc_ex < 'a > (&'a mut self, peer: i32, object: impl AsObjectArg < crate::classes::Object >, method: impl AsArg < StringName > + 'a,) -> ExRpc < 'a > {
            ExRpc::new(self, peer, object, method,)
        }
        pub fn object_configuration_add(&mut self, object: impl AsObjectArg < crate::classes::Object >, configuration: &Variant,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, ObjectArg < crate::classes::Object >, RefArg < 'a0, Variant >);
            let args = (object.as_object_arg(), RefArg::new(configuration),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5200usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerApi", "object_configuration_add", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn object_configuration_remove(&mut self, object: impl AsObjectArg < crate::classes::Object >, configuration: &Variant,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, ObjectArg < crate::classes::Object >, RefArg < 'a0, Variant >);
            let args = (object.as_object_arg(), RefArg::new(configuration),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5201usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerApi", "object_configuration_remove", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_peers(&mut self,) -> PackedInt32Array {
            type CallSig = (PackedInt32Array,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5202usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerApi", "get_peers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_default_interface(interface_name: impl AsArg < StringName >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, StringName >);
            let args = (interface_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5203usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerApi", "set_default_interface", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn get_default_interface() -> StringName {
            type CallSig = (StringName,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5204usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerApi", "get_default_interface", std::ptr::null_mut(), None, args,)
            }
        }
        pub fn create_default_interface() -> Option < Gd < crate::classes::MultiplayerApi > > {
            type CallSig = (Option < Gd < crate::classes::MultiplayerApi > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5205usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerApi", "create_default_interface", std::ptr::null_mut(), None, args,)
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
    impl crate::obj::GodotClass for MultiplayerApi {
        type Base = crate::classes::RefCounted;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"MultiplayerAPI"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MultiplayerApi {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for MultiplayerApi {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for MultiplayerApi {
        
    }
    impl std::ops::Deref for MultiplayerApi {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MultiplayerApi {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`MultiplayerApi`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_MultiplayerApi {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::MultiplayerApi > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`MultiplayerApi::rpc_ex`][super::MultiplayerApi::rpc_ex]."]
#[must_use]
pub struct ExRpc < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::MultiplayerApi, peer: i32, object: ObjectCow < crate::classes::Object >, method: CowArg < 'a, StringName >, arguments: CowArg < 'a, VariantArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExRpc < 'a > {
    fn new(surround_object: &'a mut re_export::MultiplayerApi, peer: i32, object: impl AsObjectArg < crate::classes::Object >, method: impl AsArg < StringName > + 'a,) -> Self {
        let arguments = Array::new();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, peer: peer, object: object.consume_arg(), method: method.into_arg(), arguments: CowArg::Owned(arguments),
        }
    }
    #[inline]
    pub fn arguments(self, arguments: &'a VariantArray) -> Self {
        Self {
            arguments: CowArg::Borrowed(arguments), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, peer, object, method, arguments,
        }
        = self;
        re_export::MultiplayerApi::rpc_full(surround_object, peer, object.cow_as_object_arg(), method, arguments.cow_as_arg(),)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `RPCMode`."]
pub struct RpcMode {
    ord: i32
}
impl RpcMode {
    #[doc(alias = "RPC_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `RPC_MODE_DISABLED`"]
    pub const DISABLED: RpcMode = RpcMode {
        ord: 0i32
    };
    #[doc(alias = "RPC_MODE_ANY_PEER")]
    #[doc = "Godot enumerator name: `RPC_MODE_ANY_PEER`"]
    pub const ANY_PEER: RpcMode = RpcMode {
        ord: 1i32
    };
    #[doc(alias = "RPC_MODE_AUTHORITY")]
    #[doc = "Godot enumerator name: `RPC_MODE_AUTHORITY`"]
    pub const AUTHORITY: RpcMode = RpcMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for RpcMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("RpcMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for RpcMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self {
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
            Self::DISABLED => "DISABLED", Self::ANY_PEER => "ANY_PEER", Self::AUTHORITY => "AUTHORITY", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISABLED => "RPC_MODE_DISABLED", Self::ANY_PEER => "RPC_MODE_ANY_PEER", Self::AUTHORITY => "RPC_MODE_AUTHORITY", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for RpcMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for RpcMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for RpcMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}