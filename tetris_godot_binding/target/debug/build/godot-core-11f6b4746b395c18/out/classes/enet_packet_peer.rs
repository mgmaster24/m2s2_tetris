#![doc = "Sidecar module for class [`ENetPacketPeer`][crate::classes::ENetPacketPeer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ENetPacketPeer` enums](https://docs.godotengine.org/en/stable/classes/class_enetpacketpeer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ENetPacketPeer.`\n\nInherits [`PacketPeer`][crate::classes::PacketPeer].\n\nRelated symbols:\n\n* [`enet_packet_peer`][crate::classes::enet_packet_peer]: sidecar module with related enum/flag types\n* [`IENetPacketPeer`][crate::classes::IENetPacketPeer]: virtual methods\n\n\nSee also [Godot docs for `ENetPacketPeer`](https://docs.godotengine.org/en/stable/classes/class_enetpacketpeer.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<ENetPacketPeer>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ENetPacketPeer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`ENetPacketPeer`][crate::classes::ENetPacketPeer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ENetPacketPeer` methods](https://docs.godotengine.org/en/stable/classes/class_enetpacketpeer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IENetPacketPeer: crate::obj::GodotClass < Base = ENetPacketPeer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl ENetPacketPeer {
        pub(crate) fn peer_disconnect_full(&mut self, data: i32,) {
            type CallSig = ((), i32);
            let args = (data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2837usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "peer_disconnect", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::peer_disconnect_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn peer_disconnect(&mut self,) {
            self.peer_disconnect_ex() . done()
        }
        #[inline]
        pub fn peer_disconnect_ex < 'a > (&'a mut self,) -> ExPeerDisconnect < 'a > {
            ExPeerDisconnect::new(self,)
        }
        pub(crate) fn peer_disconnect_later_full(&mut self, data: i32,) {
            type CallSig = ((), i32);
            let args = (data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2838usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "peer_disconnect_later", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::peer_disconnect_later_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn peer_disconnect_later(&mut self,) {
            self.peer_disconnect_later_ex() . done()
        }
        #[inline]
        pub fn peer_disconnect_later_ex < 'a > (&'a mut self,) -> ExPeerDisconnectLater < 'a > {
            ExPeerDisconnectLater::new(self,)
        }
        pub(crate) fn peer_disconnect_now_full(&mut self, data: i32,) {
            type CallSig = ((), i32);
            let args = (data,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2839usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "peer_disconnect_now", self.object_ptr, self.__checked_id(), args,)
            }
        }
        #[doc = "To set the default parameters, use [`Self::peer_disconnect_now_ex`] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[inline]
        pub fn peer_disconnect_now(&mut self,) {
            self.peer_disconnect_now_ex() . done()
        }
        #[inline]
        pub fn peer_disconnect_now_ex < 'a > (&'a mut self,) -> ExPeerDisconnectNow < 'a > {
            ExPeerDisconnectNow::new(self,)
        }
        pub fn ping(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2840usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "ping", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn ping_interval(&mut self, ping_interval: i32,) {
            type CallSig = ((), i32);
            let args = (ping_interval,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2841usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "ping_interval", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn reset(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2842usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "reset", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn send(&mut self, channel: i32, packet: &PackedByteArray, flags: i32,) -> crate::global::Error {
            type CallSig < 'a0, > = (crate::global::Error, i32, RefArg < 'a0, PackedByteArray >, i32);
            let args = (channel, RefArg::new(packet), flags,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2843usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "send", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn throttle_configure(&mut self, interval: i32, acceleration: i32, deceleration: i32,) {
            type CallSig = ((), i32, i32, i32);
            let args = (interval, acceleration, deceleration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2844usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "throttle_configure", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_timeout(&mut self, timeout: i32, timeout_min: i32, timeout_max: i32,) {
            type CallSig = ((), i32, i32, i32);
            let args = (timeout, timeout_min, timeout_max,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2845usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "set_timeout", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_remote_address(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2846usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "get_remote_address", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_remote_port(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2847usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "get_remote_port", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_statistic(&mut self, statistic: crate::classes::enet_packet_peer::PeerStatistic,) -> f64 {
            type CallSig = (f64, crate::classes::enet_packet_peer::PeerStatistic);
            let args = (statistic,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2848usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "get_statistic", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_state(&self,) -> crate::classes::enet_packet_peer::PeerState {
            type CallSig = (crate::classes::enet_packet_peer::PeerState,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2849usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "get_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_channels(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2850usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "get_channels", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_active(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(2851usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "ENetPacketPeer", "is_active", self.object_ptr, self.__checked_id(), args,)
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
        pub const PACKET_LOSS_SCALE: i32 = 65536i32;
        pub const PACKET_THROTTLE_SCALE: i32 = 32i32;
        pub const FLAG_RELIABLE: i32 = 1i32;
        pub const FLAG_UNSEQUENCED: i32 = 2i32;
        pub const FLAG_UNRELIABLE_FRAGMENT: i32 = 8i32;
        
    }
    impl crate::obj::GodotClass for ENetPacketPeer {
        type Base = crate::classes::PacketPeer;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"ENetPacketPeer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ENetPacketPeer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PacketPeer > for ENetPacketPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for ENetPacketPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ENetPacketPeer {
        
    }
    impl std::ops::Deref for ENetPacketPeer {
        type Target = crate::classes::PacketPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ENetPacketPeer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ENetPacketPeer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_ENetPacketPeer {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ENetPacketPeer > for $Class {
                
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
#[doc = "Default-param extender for [`ENetPacketPeer::peer_disconnect_ex`][super::ENetPacketPeer::peer_disconnect_ex]."]
#[must_use]
pub struct ExPeerDisconnect < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ENetPacketPeer, data: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPeerDisconnect < 'a > {
    fn new(surround_object: &'a mut re_export::ENetPacketPeer,) -> Self {
        let data = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, data: data,
        }
    }
    #[inline]
    pub fn data(self, data: i32) -> Self {
        Self {
            data: data, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, data,
        }
        = self;
        re_export::ENetPacketPeer::peer_disconnect_full(surround_object, data,)
    }
}
#[doc = "Default-param extender for [`ENetPacketPeer::peer_disconnect_later_ex`][super::ENetPacketPeer::peer_disconnect_later_ex]."]
#[must_use]
pub struct ExPeerDisconnectLater < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ENetPacketPeer, data: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPeerDisconnectLater < 'a > {
    fn new(surround_object: &'a mut re_export::ENetPacketPeer,) -> Self {
        let data = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, data: data,
        }
    }
    #[inline]
    pub fn data(self, data: i32) -> Self {
        Self {
            data: data, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, data,
        }
        = self;
        re_export::ENetPacketPeer::peer_disconnect_later_full(surround_object, data,)
    }
}
#[doc = "Default-param extender for [`ENetPacketPeer::peer_disconnect_now_ex`][super::ENetPacketPeer::peer_disconnect_now_ex]."]
#[must_use]
pub struct ExPeerDisconnectNow < 'a > {
    _phantom: std::marker::PhantomData < &'a() >, surround_object: &'a mut re_export::ENetPacketPeer, data: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'a > ExPeerDisconnectNow < 'a > {
    fn new(surround_object: &'a mut re_export::ENetPacketPeer,) -> Self {
        let data = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, data: data,
        }
    }
    #[inline]
    pub fn data(self, data: i32) -> Self {
        Self {
            data: data, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, data,
        }
        = self;
        re_export::ENetPacketPeer::peer_disconnect_now_full(surround_object, data,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PeerState {
    ord: i32
}
impl PeerState {
    #[doc(alias = "STATE_DISCONNECTED")]
    #[doc = "Godot enumerator name: `STATE_DISCONNECTED`"]
    pub const DISCONNECTED: PeerState = PeerState {
        ord: 0i32
    };
    #[doc(alias = "STATE_CONNECTING")]
    #[doc = "Godot enumerator name: `STATE_CONNECTING`"]
    pub const CONNECTING: PeerState = PeerState {
        ord: 1i32
    };
    #[doc(alias = "STATE_ACKNOWLEDGING_CONNECT")]
    #[doc = "Godot enumerator name: `STATE_ACKNOWLEDGING_CONNECT`"]
    pub const ACKNOWLEDGING_CONNECT: PeerState = PeerState {
        ord: 2i32
    };
    #[doc(alias = "STATE_CONNECTION_PENDING")]
    #[doc = "Godot enumerator name: `STATE_CONNECTION_PENDING`"]
    pub const CONNECTION_PENDING: PeerState = PeerState {
        ord: 3i32
    };
    #[doc(alias = "STATE_CONNECTION_SUCCEEDED")]
    #[doc = "Godot enumerator name: `STATE_CONNECTION_SUCCEEDED`"]
    pub const CONNECTION_SUCCEEDED: PeerState = PeerState {
        ord: 4i32
    };
    #[doc(alias = "STATE_CONNECTED")]
    #[doc = "Godot enumerator name: `STATE_CONNECTED`"]
    pub const CONNECTED: PeerState = PeerState {
        ord: 5i32
    };
    #[doc(alias = "STATE_DISCONNECT_LATER")]
    #[doc = "Godot enumerator name: `STATE_DISCONNECT_LATER`"]
    pub const DISCONNECT_LATER: PeerState = PeerState {
        ord: 6i32
    };
    #[doc(alias = "STATE_DISCONNECTING")]
    #[doc = "Godot enumerator name: `STATE_DISCONNECTING`"]
    pub const DISCONNECTING: PeerState = PeerState {
        ord: 7i32
    };
    #[doc(alias = "STATE_ACKNOWLEDGING_DISCONNECT")]
    #[doc = "Godot enumerator name: `STATE_ACKNOWLEDGING_DISCONNECT`"]
    pub const ACKNOWLEDGING_DISCONNECT: PeerState = PeerState {
        ord: 8i32
    };
    #[doc(alias = "STATE_ZOMBIE")]
    #[doc = "Godot enumerator name: `STATE_ZOMBIE`"]
    pub const ZOMBIE: PeerState = PeerState {
        ord: 9i32
    };
    
}
impl std::fmt::Debug for PeerState {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PeerState") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PeerState {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 => Some(Self {
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
            Self::DISCONNECTED => "DISCONNECTED", Self::CONNECTING => "CONNECTING", Self::ACKNOWLEDGING_CONNECT => "ACKNOWLEDGING_CONNECT", Self::CONNECTION_PENDING => "CONNECTION_PENDING", Self::CONNECTION_SUCCEEDED => "CONNECTION_SUCCEEDED", Self::CONNECTED => "CONNECTED", Self::DISCONNECT_LATER => "DISCONNECT_LATER", Self::DISCONNECTING => "DISCONNECTING", Self::ACKNOWLEDGING_DISCONNECT => "ACKNOWLEDGING_DISCONNECT", Self::ZOMBIE => "ZOMBIE", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::DISCONNECTED => "STATE_DISCONNECTED", Self::CONNECTING => "STATE_CONNECTING", Self::ACKNOWLEDGING_CONNECT => "STATE_ACKNOWLEDGING_CONNECT", Self::CONNECTION_PENDING => "STATE_CONNECTION_PENDING", Self::CONNECTION_SUCCEEDED => "STATE_CONNECTION_SUCCEEDED", Self::CONNECTED => "STATE_CONNECTED", Self::DISCONNECT_LATER => "STATE_DISCONNECT_LATER", Self::DISCONNECTING => "STATE_DISCONNECTING", Self::ACKNOWLEDGING_DISCONNECT => "STATE_ACKNOWLEDGING_DISCONNECT", Self::ZOMBIE => "STATE_ZOMBIE", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for PeerState {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PeerState {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PeerState {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PeerStatistic {
    ord: i32
}
impl PeerStatistic {
    #[doc(alias = "PEER_PACKET_LOSS")]
    #[doc = "Godot enumerator name: `PEER_PACKET_LOSS`"]
    pub const PACKET_LOSS: PeerStatistic = PeerStatistic {
        ord: 0i32
    };
    #[doc(alias = "PEER_PACKET_LOSS_VARIANCE")]
    #[doc = "Godot enumerator name: `PEER_PACKET_LOSS_VARIANCE`"]
    pub const PACKET_LOSS_VARIANCE: PeerStatistic = PeerStatistic {
        ord: 1i32
    };
    #[doc(alias = "PEER_PACKET_LOSS_EPOCH")]
    #[doc = "Godot enumerator name: `PEER_PACKET_LOSS_EPOCH`"]
    pub const PACKET_LOSS_EPOCH: PeerStatistic = PeerStatistic {
        ord: 2i32
    };
    #[doc(alias = "PEER_ROUND_TRIP_TIME")]
    #[doc = "Godot enumerator name: `PEER_ROUND_TRIP_TIME`"]
    pub const ROUND_TRIP_TIME: PeerStatistic = PeerStatistic {
        ord: 3i32
    };
    #[doc(alias = "PEER_ROUND_TRIP_TIME_VARIANCE")]
    #[doc = "Godot enumerator name: `PEER_ROUND_TRIP_TIME_VARIANCE`"]
    pub const ROUND_TRIP_TIME_VARIANCE: PeerStatistic = PeerStatistic {
        ord: 4i32
    };
    #[doc(alias = "PEER_LAST_ROUND_TRIP_TIME")]
    #[doc = "Godot enumerator name: `PEER_LAST_ROUND_TRIP_TIME`"]
    pub const LAST_ROUND_TRIP_TIME: PeerStatistic = PeerStatistic {
        ord: 5i32
    };
    #[doc(alias = "PEER_LAST_ROUND_TRIP_TIME_VARIANCE")]
    #[doc = "Godot enumerator name: `PEER_LAST_ROUND_TRIP_TIME_VARIANCE`"]
    pub const LAST_ROUND_TRIP_TIME_VARIANCE: PeerStatistic = PeerStatistic {
        ord: 6i32
    };
    #[doc(alias = "PEER_PACKET_THROTTLE")]
    #[doc = "Godot enumerator name: `PEER_PACKET_THROTTLE`"]
    pub const PACKET_THROTTLE: PeerStatistic = PeerStatistic {
        ord: 7i32
    };
    #[doc(alias = "PEER_PACKET_THROTTLE_LIMIT")]
    #[doc = "Godot enumerator name: `PEER_PACKET_THROTTLE_LIMIT`"]
    pub const PACKET_THROTTLE_LIMIT: PeerStatistic = PeerStatistic {
        ord: 8i32
    };
    #[doc(alias = "PEER_PACKET_THROTTLE_COUNTER")]
    #[doc = "Godot enumerator name: `PEER_PACKET_THROTTLE_COUNTER`"]
    pub const PACKET_THROTTLE_COUNTER: PeerStatistic = PeerStatistic {
        ord: 9i32
    };
    #[doc(alias = "PEER_PACKET_THROTTLE_EPOCH")]
    #[doc = "Godot enumerator name: `PEER_PACKET_THROTTLE_EPOCH`"]
    pub const PACKET_THROTTLE_EPOCH: PeerStatistic = PeerStatistic {
        ord: 10i32
    };
    #[doc(alias = "PEER_PACKET_THROTTLE_ACCELERATION")]
    #[doc = "Godot enumerator name: `PEER_PACKET_THROTTLE_ACCELERATION`"]
    pub const PACKET_THROTTLE_ACCELERATION: PeerStatistic = PeerStatistic {
        ord: 11i32
    };
    #[doc(alias = "PEER_PACKET_THROTTLE_DECELERATION")]
    #[doc = "Godot enumerator name: `PEER_PACKET_THROTTLE_DECELERATION`"]
    pub const PACKET_THROTTLE_DECELERATION: PeerStatistic = PeerStatistic {
        ord: 12i32
    };
    #[doc(alias = "PEER_PACKET_THROTTLE_INTERVAL")]
    #[doc = "Godot enumerator name: `PEER_PACKET_THROTTLE_INTERVAL`"]
    pub const PACKET_THROTTLE_INTERVAL: PeerStatistic = PeerStatistic {
        ord: 13i32
    };
    
}
impl std::fmt::Debug for PeerStatistic {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PeerStatistic") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PeerStatistic {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 | ord @ 6i32 | ord @ 7i32 | ord @ 8i32 | ord @ 9i32 | ord @ 10i32 | ord @ 11i32 | ord @ 12i32 | ord @ 13i32 => Some(Self {
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
            Self::PACKET_LOSS => "PACKET_LOSS", Self::PACKET_LOSS_VARIANCE => "PACKET_LOSS_VARIANCE", Self::PACKET_LOSS_EPOCH => "PACKET_LOSS_EPOCH", Self::ROUND_TRIP_TIME => "ROUND_TRIP_TIME", Self::ROUND_TRIP_TIME_VARIANCE => "ROUND_TRIP_TIME_VARIANCE", Self::LAST_ROUND_TRIP_TIME => "LAST_ROUND_TRIP_TIME", Self::LAST_ROUND_TRIP_TIME_VARIANCE => "LAST_ROUND_TRIP_TIME_VARIANCE", Self::PACKET_THROTTLE => "PACKET_THROTTLE", Self::PACKET_THROTTLE_LIMIT => "PACKET_THROTTLE_LIMIT", Self::PACKET_THROTTLE_COUNTER => "PACKET_THROTTLE_COUNTER", Self::PACKET_THROTTLE_EPOCH => "PACKET_THROTTLE_EPOCH", Self::PACKET_THROTTLE_ACCELERATION => "PACKET_THROTTLE_ACCELERATION", Self::PACKET_THROTTLE_DECELERATION => "PACKET_THROTTLE_DECELERATION", Self::PACKET_THROTTLE_INTERVAL => "PACKET_THROTTLE_INTERVAL", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::PACKET_LOSS => "PEER_PACKET_LOSS", Self::PACKET_LOSS_VARIANCE => "PEER_PACKET_LOSS_VARIANCE", Self::PACKET_LOSS_EPOCH => "PEER_PACKET_LOSS_EPOCH", Self::ROUND_TRIP_TIME => "PEER_ROUND_TRIP_TIME", Self::ROUND_TRIP_TIME_VARIANCE => "PEER_ROUND_TRIP_TIME_VARIANCE", Self::LAST_ROUND_TRIP_TIME => "PEER_LAST_ROUND_TRIP_TIME", Self::LAST_ROUND_TRIP_TIME_VARIANCE => "PEER_LAST_ROUND_TRIP_TIME_VARIANCE", Self::PACKET_THROTTLE => "PEER_PACKET_THROTTLE", Self::PACKET_THROTTLE_LIMIT => "PEER_PACKET_THROTTLE_LIMIT", Self::PACKET_THROTTLE_COUNTER => "PEER_PACKET_THROTTLE_COUNTER", Self::PACKET_THROTTLE_EPOCH => "PEER_PACKET_THROTTLE_EPOCH", Self::PACKET_THROTTLE_ACCELERATION => "PEER_PACKET_THROTTLE_ACCELERATION", Self::PACKET_THROTTLE_DECELERATION => "PEER_PACKET_THROTTLE_DECELERATION", Self::PACKET_THROTTLE_INTERVAL => "PEER_PACKET_THROTTLE_INTERVAL", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for PeerStatistic {
    type Via = i32;
    
}
impl crate::meta::ToGodot for PeerStatistic {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PeerStatistic {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}