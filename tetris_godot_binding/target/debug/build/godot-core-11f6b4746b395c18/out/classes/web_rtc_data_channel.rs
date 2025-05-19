#![doc = "Sidecar module for class [`WebRtcDataChannel`][crate::classes::WebRtcDataChannel].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `WebRTCDataChannel` enums](https://docs.godotengine.org/en/stable/classes/class_webrtcdatachannel.html#enumerations).\n\n"]
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
    #[doc = "Godot class `WebRTCDataChannel.`\n\nInherits [`PacketPeer`][crate::classes::PacketPeer].\n\nRelated symbols:\n\n* [`web_rtc_data_channel`][crate::classes::web_rtc_data_channel]: sidecar module with related enum/flag types\n* [`IWebRtcDataChannel`][crate::classes::IWebRtcDataChannel]: virtual methods\n\n\nSee also [Godot docs for `WebRTCDataChannel`](https://docs.godotengine.org/en/stable/classes/class_webrtcdatachannel.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<WebRtcDataChannel>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct WebRtcDataChannel {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`WebRtcDataChannel`][crate::classes::WebRtcDataChannel].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `WebRTCDataChannel` methods](https://docs.godotengine.org/en/stable/classes/class_webrtcdatachannel.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IWebRtcDataChannel: crate::obj::GodotClass < Base = WebRtcDataChannel > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl WebRtcDataChannel {
        pub fn poll(&mut self,) -> crate::global::Error {
            type CallSig = (crate::global::Error,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10242usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "poll", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn close(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10243usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "close", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn was_string_packet(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10244usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "was_string_packet", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_write_mode(&mut self, write_mode: crate::classes::web_rtc_data_channel::WriteMode,) {
            type CallSig = ((), crate::classes::web_rtc_data_channel::WriteMode);
            let args = (write_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10245usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "set_write_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_write_mode(&self,) -> crate::classes::web_rtc_data_channel::WriteMode {
            type CallSig = (crate::classes::web_rtc_data_channel::WriteMode,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10246usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "get_write_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_ready_state(&self,) -> crate::classes::web_rtc_data_channel::ChannelState {
            type CallSig = (crate::classes::web_rtc_data_channel::ChannelState,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10247usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "get_ready_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_label(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10248usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "get_label", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_ordered(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10249usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "is_ordered", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_id(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10250usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "get_id", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_packet_life_time(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10251usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "get_max_packet_life_time", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_max_retransmits(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10252usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "get_max_retransmits", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_protocol(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10253usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "get_protocol", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_negotiated(&self,) -> bool {
            type CallSig = (bool,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10254usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "is_negotiated", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_buffered_amount(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10255usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "get_buffered_amount", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for WebRtcDataChannel {
        type Base = crate::classes::PacketPeer;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"WebRTCDataChannel"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for WebRtcDataChannel {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PacketPeer > for WebRtcDataChannel {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for WebRtcDataChannel {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for WebRtcDataChannel {
        
    }
    impl std::ops::Deref for WebRtcDataChannel {
        type Target = crate::classes::PacketPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for WebRtcDataChannel {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`WebRtcDataChannel`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_WebRtcDataChannel {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::WebRtcDataChannel > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct WriteMode {
    ord: i32
}
impl WriteMode {
    #[doc(alias = "WRITE_MODE_TEXT")]
    #[doc = "Godot enumerator name: `WRITE_MODE_TEXT`"]
    pub const TEXT: WriteMode = WriteMode {
        ord: 0i32
    };
    #[doc(alias = "WRITE_MODE_BINARY")]
    #[doc = "Godot enumerator name: `WRITE_MODE_BINARY`"]
    pub const BINARY: WriteMode = WriteMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for WriteMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("WriteMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for WriteMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
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
            Self::TEXT => "TEXT", Self::BINARY => "BINARY", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::TEXT => "WRITE_MODE_TEXT", Self::BINARY => "WRITE_MODE_BINARY", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for WriteMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for WriteMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for WriteMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ChannelState {
    ord: i32
}
impl ChannelState {
    #[doc(alias = "STATE_CONNECTING")]
    #[doc = "Godot enumerator name: `STATE_CONNECTING`"]
    pub const CONNECTING: ChannelState = ChannelState {
        ord: 0i32
    };
    #[doc(alias = "STATE_OPEN")]
    #[doc = "Godot enumerator name: `STATE_OPEN`"]
    pub const OPEN: ChannelState = ChannelState {
        ord: 1i32
    };
    #[doc(alias = "STATE_CLOSING")]
    #[doc = "Godot enumerator name: `STATE_CLOSING`"]
    pub const CLOSING: ChannelState = ChannelState {
        ord: 2i32
    };
    #[doc(alias = "STATE_CLOSED")]
    #[doc = "Godot enumerator name: `STATE_CLOSED`"]
    pub const CLOSED: ChannelState = ChannelState {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ChannelState {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ChannelState") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ChannelState {
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
            Self::CONNECTING => "CONNECTING", Self::OPEN => "OPEN", Self::CLOSING => "CLOSING", Self::CLOSED => "CLOSED", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::CONNECTING => "STATE_CONNECTING", Self::OPEN => "STATE_OPEN", Self::CLOSING => "STATE_CLOSING", Self::CLOSED => "STATE_CLOSED", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for ChannelState {
    type Via = i32;
    
}
impl crate::meta::ToGodot for ChannelState {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ChannelState {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}