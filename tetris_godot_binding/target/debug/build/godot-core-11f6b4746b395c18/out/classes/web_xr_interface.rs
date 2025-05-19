#![doc = "Sidecar module for class [`WebXrInterface`][crate::classes::WebXrInterface].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `WebXRInterface` enums](https://docs.godotengine.org/en/stable/classes/class_webxrinterface.html#enumerations).\n\n"]
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
    #[doc = "Godot class `WebXRInterface.`\n\nInherits [`XrInterface`][crate::classes::XrInterface].\n\nRelated symbols:\n\n* [`web_xr_interface`][crate::classes::web_xr_interface]: sidecar module with related enum/flag types\n* [`IWebXrInterface`][crate::classes::IWebXrInterface]: virtual methods\n\n\nSee also [Godot docs for `WebXRInterface`](https://docs.godotengine.org/en/stable/classes/class_webxrinterface.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<WebXrInterface>` instances via Godot APIs."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct WebXrInterface {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "Virtual methods for class [`WebXrInterface`][crate::classes::WebXrInterface].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `WebXRInterface` methods](https://docs.godotengine.org/en/stable/classes/class_webxrinterface.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IWebXrInterface: crate::obj::GodotClass < Base = WebXrInterface > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl WebXrInterface {
        pub fn is_session_supported(&mut self, session_mode: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (session_mode.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10319usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebXrInterface", "is_session_supported", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_session_mode(&mut self, session_mode: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (session_mode.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10320usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebXrInterface", "set_session_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_session_mode(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10321usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_session_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_required_features(&mut self, required_features: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (required_features.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10322usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebXrInterface", "set_required_features", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_required_features(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10323usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_required_features", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_optional_features(&mut self, optional_features: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (optional_features.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10324usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebXrInterface", "set_optional_features", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_optional_features(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10325usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_optional_features", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_reference_space_type(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10326usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_reference_space_type", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_enabled_features(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10327usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_enabled_features", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_requested_reference_space_types(&mut self, requested_reference_space_types: impl AsArg < GString >,) {
            type CallSig < 'a0, > = ((), CowArg < 'a0, GString >);
            let args = (requested_reference_space_types.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10328usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebXrInterface", "set_requested_reference_space_types", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_requested_reference_space_types(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10329usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_requested_reference_space_types", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn is_input_source_active(&self, input_source_id: i32,) -> bool {
            type CallSig = (bool, i32);
            let args = (input_source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10330usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebXrInterface", "is_input_source_active", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_input_source_tracker(&self, input_source_id: i32,) -> Option < Gd < crate::classes::XrControllerTracker > > {
            type CallSig = (Option < Gd < crate::classes::XrControllerTracker > >, i32);
            let args = (input_source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10331usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_input_source_tracker", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_input_source_target_ray_mode(&self, input_source_id: i32,) -> crate::classes::web_xr_interface::TargetRayMode {
            type CallSig = (crate::classes::web_xr_interface::TargetRayMode, i32);
            let args = (input_source_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10332usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_input_source_target_ray_mode", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_visibility_state(&self,) -> GString {
            type CallSig = (GString,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10333usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_visibility_state", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_display_refresh_rate(&self,) -> f32 {
            type CallSig = (f32,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10334usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_display_refresh_rate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn set_display_refresh_rate(&mut self, refresh_rate: f32,) {
            type CallSig = ((), f32);
            let args = (refresh_rate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10335usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebXrInterface", "set_display_refresh_rate", self.object_ptr, self.__checked_id(), args,)
            }
        }
        pub fn get_available_display_refresh_rates(&self,) -> VariantArray {
            type CallSig = (VariantArray,);
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10336usize);
                < CallSig as PtrcallSignatureTuple > ::out_class_ptrcall(method_bind, "WebXrInterface", "get_available_display_refresh_rates", self.object_ptr, self.__checked_id(), args,)
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
    impl crate::obj::GodotClass for WebXrInterface {
        type Base = crate::classes::XrInterface;
        fn class_name() -> ClassName {
            static CLASS_NAME: std::sync::OnceLock < ClassName > = std::sync::OnceLock::new();
            let name: &'static ClassName = CLASS_NAME.get_or_init(|| ClassName::alloc_next_ascii(c"WebXRInterface"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for WebXrInterface {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::XrInterface > for WebXrInterface {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for WebXrInterface {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for WebXrInterface {
        
    }
    impl std::ops::Deref for WebXrInterface {
        type Target = crate::classes::XrInterface;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for WebXrInterface {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`WebXrInterface`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !unsafe_inherits_transitive_WebXrInterface {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::WebXrInterface > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::XrInterface > for $Class {
                
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
pub struct TargetRayMode {
    ord: i32
}
impl TargetRayMode {
    #[doc(alias = "TARGET_RAY_MODE_UNKNOWN")]
    #[doc = "Godot enumerator name: `TARGET_RAY_MODE_UNKNOWN`"]
    pub const UNKNOWN: TargetRayMode = TargetRayMode {
        ord: 0i32
    };
    #[doc(alias = "TARGET_RAY_MODE_GAZE")]
    #[doc = "Godot enumerator name: `TARGET_RAY_MODE_GAZE`"]
    pub const GAZE: TargetRayMode = TargetRayMode {
        ord: 1i32
    };
    #[doc(alias = "TARGET_RAY_MODE_TRACKED_POINTER")]
    #[doc = "Godot enumerator name: `TARGET_RAY_MODE_TRACKED_POINTER`"]
    pub const TRACKED_POINTER: TargetRayMode = TargetRayMode {
        ord: 2i32
    };
    #[doc(alias = "TARGET_RAY_MODE_SCREEN")]
    #[doc = "Godot enumerator name: `TARGET_RAY_MODE_SCREEN`"]
    pub const SCREEN: TargetRayMode = TargetRayMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for TargetRayMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TargetRayMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TargetRayMode {
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
            Self::UNKNOWN => "UNKNOWN", Self::GAZE => "GAZE", Self::TRACKED_POINTER => "TRACKED_POINTER", Self::SCREEN => "SCREEN", _ => "",
        }
    }
    fn godot_name(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match * self {
            Self::UNKNOWN => "TARGET_RAY_MODE_UNKNOWN", Self::GAZE => "TARGET_RAY_MODE_GAZE", Self::TRACKED_POINTER => "TARGET_RAY_MODE_TRACKED_POINTER", Self::SCREEN => "TARGET_RAY_MODE_SCREEN", _ => self.as_str(),
        }
    }
}
impl crate::meta::GodotConvert for TargetRayMode {
    type Via = i32;
    
}
impl crate::meta::ToGodot for TargetRayMode {
    type ToVia < 'v > = i32;
    fn to_godot(&self) -> Self::ToVia < '_ > {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TargetRayMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via))
    }
}