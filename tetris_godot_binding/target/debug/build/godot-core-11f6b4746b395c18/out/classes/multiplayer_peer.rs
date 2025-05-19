#![doc = "Sidecar module for class [`MultiplayerPeer`][crate::classes::MultiplayerPeer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MultiplayerPeer` enums](https://docs.godotengine.org/en/stable/classes/class_multiplayerpeer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MultiplayerPeer.`\n\nInherits [`PacketPeer`][crate::classes::PacketPeer].\n\nRelated symbols:\n\n* [`multiplayer_peer`][crate::classes::multiplayer_peer]: sidecar module with related enum/flag types\n* [`IMultiplayerPeer`][crate::classes::IMultiplayerPeer]: virtual methods\n\n\nSee also [Godot docs for `MultiplayerPeer`](https://docs.godotengine.org/en/stable/classes/class_multiplayerpeer.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<MultiplayerPeer>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MultiplayerPeer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`MultiplayerPeer`][crate::classes::MultiplayerPeer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `MultiplayerPeer` methods](https://docs.godotengine.org/en/stable/classes/class_multiplayerpeer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMultiplayerPeer: crate::obj::GodotClass < Base = MultiplayerPeer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl MultiplayerPeer {
        pub fn set_transfer_channel(&mut self, channel: i32,) {
            type CallSig = ((), i32);
            let args = (channel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5206usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "set_transfer_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transfer_channel(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5207usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "get_transfer_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_transfer_mode(&mut self, mode: crate::classes::multiplayer_peer::TransferMode,) {
            type CallSig = ((), crate::classes::multiplayer_peer::TransferMode);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5208usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "set_transfer_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_transfer_mode(&self,) -> crate::classes::multiplayer_peer::TransferMode {
            type CallSig = (crate::classes::multiplayer_peer::TransferMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5209usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "get_transfer_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_target_peer(&mut self, id: i32,) {
            type CallSig = ((), i32);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5210usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "set_target_peer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_packet_peer(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5211usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "get_packet_peer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_packet_channel(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5212usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "get_packet_channel", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_packet_mode(&self,) -> crate::classes::multiplayer_peer::TransferMode {
            type CallSig = (crate::classes::multiplayer_peer::TransferMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5213usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "get_packet_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn poll(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5214usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "poll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn close(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5215usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "close", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub(crate) fn disconnect_peer_full(&mut self, peer: i32, force: bool,) {
            type CallSig = ((), i32, bool);
            let args = (peer, force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5216usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "disconnect_peer", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::disconnect_peer_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn disconnect_peer(&mut self, peer: i32,) {
            self.disconnect_peer_ex(peer,) . done()
        }
        #[inline]
        pub fn disconnect_peer_ex < 'a > (&'a mut self, peer: i32,) -> ExDisconnectPeer < 'a > {
            ExDisconnectPeer::new(self, peer,)
        }
        pub fn get_connection_status(&self,) -> crate::classes::multiplayer_peer::ConnectionStatus {
            type CallSig = (crate::classes::multiplayer_peer::ConnectionStatus,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5217usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "get_connection_status", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_unique_id(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5218usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "get_unique_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn generate_unique_id(&self,) -> u32 {
            type CallSig = (u32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5219usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "generate_unique_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_refuse_new_connections(&mut self, enable: bool,) {
            type CallSig = ((), bool);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5220usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "set_refuse_new_connections", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_refusing_new_connections(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5221usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "is_refusing_new_connections", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_server_relay_supported(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5222usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "is_server_relay_supported", self.object_ptr, self.__checked_id(), args,)
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
        pub const TARGET_PEER_BROADCAST: i32 = 0i32;
        pub const TARGET_PEER_SERVER: i32 = 1i32;
        
    }
    impl crate::obj::GodotClass for MultiplayerPeer {
        type Base = crate::classes::PacketPeer;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"MultiplayerPeer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MultiplayerPeer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PacketPeer > for MultiplayerPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for MultiplayerPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for MultiplayerPeer {
        
    }
    impl std::ops::Deref for MultiplayerPeer {
        type Target = crate::classes::PacketPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MultiplayerPeer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`MultiplayerPeer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_MultiplayerPeer {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::MultiplayerPeer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::PacketPeer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`MultiplayerPeer::disconnect_peer_ex`][super::MultiplayerPeer::disconnect_peer_ex]."]
#[must_use]
pub struct ExDisconnectPeer < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::MultiplayerPeer, peer: i32, force: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExDisconnectPeer < 'a > {
    fn new(surround_object: &'a mut re_export::MultiplayerPeer, peer: i32,) -> Self {
        let force = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, peer: peer, force: force,
        }
    }
    #[inline]
    pub fn force(self, force: bool) -> Self {
        Self {
            force: force, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, peer, force,
        }
        = self;
        re_export::MultiplayerPeer::disconnect_peer_full(surround_object, peer, force,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ConnectionStatus {
    ord: i32
}
impl ConnectionStatus {
    #[doc(alias = "CONNECTION_DISCONNECTED")]
    #[doc = "Godot enumerator name: `CONNECTION_DISCONNECTED`"]
    pub const DISCONNECTED: ConnectionStatus = ConnectionStatus {
        ord: 0i32
    };
    #[doc(alias = "CONNECTION_CONNECTING")]
    #[doc = "Godot enumerator name: `CONNECTION_CONNECTING`"]
    pub const CONNECTING: ConnectionStatus = ConnectionStatus {
        ord: 1i32
    };
    #[doc(alias = "CONNECTION_CONNECTED")]
    #[doc = "Godot enumerator name: `CONNECTION_CONNECTED`"]
    pub const CONNECTED: ConnectionStatus = ConnectionStatus {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ConnectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ConnectionStatus") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ConnectionStatus {
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
            Self::DISCONNECTED => "DISCONNECTED", Self::CONNECTING => "CONNECTING", Self::CONNECTED => "CONNECTED", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISCONNECTED => "CONNECTION_DISCONNECTED", Self::CONNECTING => "CONNECTION_CONNECTING", Self::CONNECTED => "CONNECTION_CONNECTED", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for ConnectionStatus {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ConnectionStatus {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ConnectionStatus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TransferMode {
    ord: i32
}
impl TransferMode {
    #[doc(alias = "TRANSFER_MODE_UNRELIABLE")]
    #[doc = "Godot enumerator name: `TRANSFER_MODE_UNRELIABLE`"]
    pub const UNRELIABLE: TransferMode = TransferMode {
        ord: 0i32
    };
    #[doc(alias = "TRANSFER_MODE_UNRELIABLE_ORDERED")]
    #[doc = "Godot enumerator name: `TRANSFER_MODE_UNRELIABLE_ORDERED`"]
    pub const UNRELIABLE_ORDERED: TransferMode = TransferMode {
        ord: 1i32
    };
    #[doc(alias = "TRANSFER_MODE_RELIABLE")]
    #[doc = "Godot enumerator name: `TRANSFER_MODE_RELIABLE`"]
    pub const RELIABLE: TransferMode = TransferMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TransferMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TransferMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TransferMode {
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
            Self::UNRELIABLE => "UNRELIABLE", Self::UNRELIABLE_ORDERED => "UNRELIABLE_ORDERED", Self::RELIABLE => "RELIABLE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::UNRELIABLE => "TRANSFER_MODE_UNRELIABLE", Self::UNRELIABLE_ORDERED => "TRANSFER_MODE_UNRELIABLE_ORDERED", Self::RELIABLE => "TRANSFER_MODE_RELIABLE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for TransferMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TransferMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TransferMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}