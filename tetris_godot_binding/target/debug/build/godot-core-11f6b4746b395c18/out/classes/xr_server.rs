#![doc = "Sidecar module for class [`XrServer`][crate::classes::XrServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `XRServer` enums](https://docs.godotengine.org/en/stable/classes/class_xrserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `XRServer.`\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`xr_server`][crate::classes::xr_server]: sidecar module with related enum/flag types\n* [`IXrServer`][crate::classes::IXrServer]: virtual methods\n\n\nSee also [Godot docs for `XRServer`](https://docs.godotengine.org/en/stable/classes/class_xrserver.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`XrServer::singleton()`][XrServer::singleton]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct XrServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`XrServer`][crate::classes::XrServer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `XRServer` methods](https://docs.godotengine.org/en/stable/classes/class_xrserver.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IXrServer: crate::obj::GodotClass < Base = XrServer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl XrServer {
        pub fn singleton() -> Gd < Self > {
            unsafe {
                let __class_name = StringName::from(c"XRServer");
                let __object_ptr = sys::interface_fn !(global_get_singleton) (__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        pub fn get_world_scale(&self,) -> f64 {
            type CallSig = (f64,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1021usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrServer", "get_world_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_world_scale(&mut self, scale: f64,) {
            type CallSig = ((), f64);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1022usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrServer", "set_world_scale", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_world_origin(&self,) -> Transform3D {
            type CallSig = (Transform3D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1023usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrServer", "get_world_origin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_world_origin(&mut self, world_origin: Transform3D,) {
            type CallSig = ((), Transform3D);
            let args = (world_origin,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1024usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrServer", "set_world_origin", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_reference_frame(&self,) -> Transform3D {
            type CallSig = (Transform3D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1025usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrServer", "get_reference_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn clear_reference_frame(&mut self,) {
            type CallSig = ((),);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1026usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrServer", "clear_reference_frame", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn center_on_hmd(&mut self, rotation_mode: crate::classes::xr_server::RotationMode, keep_height: bool,) {
            type CallSig = ((), crate::classes::xr_server::RotationMode, bool);
            let args = (rotation_mode, keep_height,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1027usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrServer", "center_on_hmd", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_hmd_transform(&mut self,) -> Transform3D {
            type CallSig = (Transform3D,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1028usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrServer", "get_hmd_transform", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_interface(&mut self, interface: impl AsObjectArg < crate::classes::XrInterface >,) {
            type CallSig = ((), ObjectArg < crate::classes::XrInterface >);
            let args = (interface.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1029usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrServer", "add_interface", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_interface_count(&self,) -> i32 {
            type CallSig = (i32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1030usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrServer", "get_interface_count", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_interface(&mut self, interface: impl AsObjectArg < crate::classes::XrInterface >,) {
            type CallSig = ((), ObjectArg < crate::classes::XrInterface >);
            let args = (interface.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1031usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrServer", "remove_interface", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_interface(&self, idx: i32,) -> Option < Gd < crate::classes::XrInterface > > {
            type CallSig = (Option < Gd < crate::classes::XrInterface > >, i32);
            let args = (idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1032usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrServer", "get_interface", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_interfaces(&self,) -> Array < Dictionary > {
            type CallSig = (Array < Dictionary >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1033usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrServer", "get_interfaces", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn find_interface(&self, name: impl AsArg < GString >,) -> Option < Gd < crate::classes::XrInterface > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::XrInterface > >, CowArg < 'a0, GString >);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1034usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrServer", "find_interface", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn add_tracker(&mut self, tracker: impl AsObjectArg < crate::classes::XrTracker >,) {
            type CallSig = ((), ObjectArg < crate::classes::XrTracker >);
            let args = (tracker.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1035usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrServer", "add_tracker", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn remove_tracker(&mut self, tracker: impl AsObjectArg < crate::classes::XrTracker >,) {
            type CallSig = ((), ObjectArg < crate::classes::XrTracker >);
            let args = (tracker.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1036usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrServer", "remove_tracker", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_trackers(&mut self, tracker_types: i32,) -> Dictionary {
            type CallSig = (Dictionary, i32);
            let args = (tracker_types,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1037usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrServer", "get_trackers", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_tracker(&self, tracker_name: impl AsArg < StringName >,) -> Option < Gd < crate::classes::XrTracker > > {
            type CallSig < 'a0, > = (Option < Gd < crate::classes::XrTracker > >, CowArg < 'a0, StringName >);
            let args = (tracker_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1038usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrServer", "get_tracker", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_primary_interface(&self,) -> Option < Gd < crate::classes::XrInterface > > {
            type CallSig = (Option < Gd < crate::classes::XrInterface > >,);
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1039usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrServer", "get_primary_interface", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_primary_interface(&mut self, interface: impl AsObjectArg < crate::classes::XrInterface >,) {
            type CallSig = ((), ObjectArg < crate::classes::XrInterface >);
            let args = (interface.as_object_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1040usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "XrServer", "set_primary_interface", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for XrServer {
        type Base = crate::classes::Object;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"XRServer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for XrServer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for XrServer {
        
    }
    impl std::ops::Deref for XrServer {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for XrServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`XrServer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_XrServer {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::XrServer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TrackerType {
    ord: i32
}
impl TrackerType {
    #[doc(alias = "TRACKER_HEAD")]
    #[doc = "Godot enumerator name: `TRACKER_HEAD`"]
    pub const HEAD: TrackerType = TrackerType {
        ord: 1i32
    };
    #[doc(alias = "TRACKER_CONTROLLER")]
    #[doc = "Godot enumerator name: `TRACKER_CONTROLLER`"]
    pub const CONTROLLER: TrackerType = TrackerType {
        ord: 2i32
    };
    #[doc(alias = "TRACKER_BASESTATION")]
    #[doc = "Godot enumerator name: `TRACKER_BASESTATION`"]
    pub const BASESTATION: TrackerType = TrackerType {
        ord: 4i32
    };
    #[doc(alias = "TRACKER_ANCHOR")]
    #[doc = "Godot enumerator name: `TRACKER_ANCHOR`"]
    pub const ANCHOR: TrackerType = TrackerType {
        ord: 8i32
    };
    #[doc(alias = "TRACKER_HAND")]
    #[doc = "Godot enumerator name: `TRACKER_HAND`"]
    pub const HAND: TrackerType = TrackerType {
        ord: 16i32
    };
    #[doc(alias = "TRACKER_BODY")]
    #[doc = "Godot enumerator name: `TRACKER_BODY`"]
    pub const BODY: TrackerType = TrackerType {
        ord: 32i32
    };
    #[doc(alias = "TRACKER_FACE")]
    #[doc = "Godot enumerator name: `TRACKER_FACE`"]
    pub const FACE: TrackerType = TrackerType {
        ord: 64i32
    };
    #[doc(alias = "TRACKER_ANY_KNOWN")]
    #[doc = "Godot enumerator name: `TRACKER_ANY_KNOWN`"]
    pub const ANY_KNOWN: TrackerType = TrackerType {
        ord: 127i32
    };
    #[doc(alias = "TRACKER_UNKNOWN")]
    #[doc = "Godot enumerator name: `TRACKER_UNKNOWN`"]
    pub const UNKNOWN: TrackerType = TrackerType {
        ord: 128i32
    };
    #[doc(alias = "TRACKER_ANY")]
    #[doc = "Godot enumerator name: `TRACKER_ANY`"]
    pub const ANY: TrackerType = TrackerType {
        ord: 255i32
    };
    
}
impl std::fmt::Debug for TrackerType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TrackerType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TrackerType {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 | ord @ 16i32 | ord @ 32i32 | ord @ 64i32 | ord @ 127i32 | ord @ 128i32 | ord @ 255i32 => Some(Self {
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
            Self::HEAD => "HEAD", Self::CONTROLLER => "CONTROLLER", Self::BASESTATION => "BASESTATION", Self::ANCHOR => "ANCHOR", Self::HAND => "HAND", Self::BODY => "BODY", Self::FACE => "FACE", Self::ANY_KNOWN => "ANY_KNOWN", Self::UNKNOWN => "UNKNOWN", Self::ANY => "ANY", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::HEAD => "TRACKER_HEAD", Self::CONTROLLER => "TRACKER_CONTROLLER", Self::BASESTATION => "TRACKER_BASESTATION", Self::ANCHOR => "TRACKER_ANCHOR", Self::HAND => "TRACKER_HAND", Self::BODY => "TRACKER_BODY", Self::FACE => "TRACKER_FACE", Self::ANY_KNOWN => "TRACKER_ANY_KNOWN", Self::UNKNOWN => "TRACKER_UNKNOWN", Self::ANY => "TRACKER_ANY", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for TrackerType {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TrackerType {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TrackerType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct RotationMode {
    ord: i32
}
impl RotationMode {
    pub const RESET_FULL_ROTATION: RotationMode = RotationMode {
        ord: 0i32
    };
    pub const RESET_BUT_KEEP_TILT: RotationMode = RotationMode {
        ord: 1i32
    };
    pub const DONT_RESET_ROTATION: RotationMode = RotationMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for RotationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("RotationMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for RotationMode {
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
            Self::RESET_FULL_ROTATION => "RESET_FULL_ROTATION", Self::RESET_BUT_KEEP_TILT => "RESET_BUT_KEEP_TILT", Self::DONT_RESET_ROTATION => "DONT_RESET_ROTATION", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        self.as_str()
    }
}
impl crate::meta::GodotConvert for RotationMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for RotationMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for RotationMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}